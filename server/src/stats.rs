#![allow(unused)]
use chrono::{Datelike, Local, Timelike};
use lazy_static::lazy_static;
use once_cell::sync::OnceCell;
use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use std::sync::mpsc::sync_channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::SyncSender;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::runtime::Handle;

use crate::Result;

use crate::notifier::NOTIFIER_HANDLE;
use crate::notifier::{Event, Notifier};
use crate::payload::{HostStat, StatsResp};

const OFFLINE_THRESHOLD: u64 = 10; // 10s 下线
const SAVE_INTERVAL: u64 = 60;

static STAT_SENDER: OnceCell<SyncSender<Cow<HostStat>>> = OnceCell::new();

pub struct StatsMgr {
    resp_json: Arc<Mutex<String>>,
    notifier_list: Arc<Mutex<Vec<Box<dyn Notifier + Send>>>>,
}

impl StatsMgr {
    pub fn new() -> Self {
        Self {
            resp_json: Arc::new(Mutex::new(String::from("{}"))),
            notifier_list: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn init(&mut self, cfg: &'static crate::config::Config) -> Result<()> {
        let mut host_map = HashMap::new();
        for host in &cfg.hosts {
            host_map.insert(String::from(&host.name), host.clone());
        }

        // load last_network_in/out
        if let Ok(file) = File::open("stats.json") {
            let stats_json: serde_json::Value = serde_json::from_reader(BufReader::new(file))?;
            if let Some(servers) = stats_json["servers"].as_array() {
                for v in servers {
                    if let (Some(name), Some(last_network_in), Some(last_network_out)) = (
                        v["name"].as_str(),
                        v["last_network_in"].as_u64(),
                        v["last_network_out"].as_u64(),
                    ) {
                        if let Some(srv) = host_map.get_mut(name) {
                            srv.last_network_in = last_network_in;
                            srv.last_network_out = last_network_out;

                            trace!(
                                "{} => last in/out ({}/{}))",
                                &name,
                                last_network_in,
                                last_network_out
                            );
                        }
                    } else {
                        error!("invalid json => {:?}", v);
                    }
                }
                trace!("load stats.json succ!");
            }
        }

        // init notifier, // 坑已挖，待填 ?plugins system?
        if cfg.tgbot.enabled {
            let o = Box::new(crate::notifier::tgbot::TGBot::new(&cfg.tgbot));
            self.notifier_list.lock().unwrap().push(o);
        }
        if cfg.wechat.enabled {
            let o = Box::new(crate::notifier::wechat::WeChat::new(&cfg.wechat));
            self.notifier_list.lock().unwrap().push(o);
        }

        let (stat_tx, stat_rx) = sync_channel(512);
        STAT_SENDER.set(stat_tx).unwrap();
        let (notifier_tx, notifier_rx) = sync_channel(512);

        let stat_dict: Arc<Mutex<HashMap<String, Cow<HostStat>>>> =
            Arc::new(Mutex::new(HashMap::new()));

        // stat_rx thread
        let stat_dict_1 = stat_dict.clone();
        let notifier_tx_1 = notifier_tx.clone();
        let vnstat = cfg.vnstat;
        thread::spawn(move || loop {
            while let Ok(mut stat) = stat_rx.recv() {
                trace!("recv stat `{:?}", stat);
                if let Some(info) = host_map.get_mut(&stat.name) {
                    let local_now = Local::now();
                    // 补齐
                    let mut stat_c = stat;
                    let mut stat_t = stat_c.to_mut();
                    stat_t.location = info.location.to_string();
                    stat_t.host_type = info.host_type.to_owned();
                    stat_t.pos = info.pos;
                    stat_t.alias = info.alias.to_owned();
                    stat_t.disabled = false;
                    stat_t.latest_ts = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs();
                    // last_network_in/out
                    if !vnstat {
                        if info.last_network_in == 0
                            || (stat_t.network_in != 0 && info.last_network_in > stat_t.network_in)
                            || (local_now.day() == info.monthstart
                                && local_now.hour() == 0
                                && local_now.minute() < 5)
                        {
                            info.last_network_in = stat_t.network_in;
                            info.last_network_out = stat_t.network_out;
                        } else {
                            stat_t.last_network_in = info.last_network_in;
                            stat_t.last_network_out = info.last_network_out;
                        }
                    }

                    // uptime str
                    let day = (stat_t.uptime as f64 / 3600.0 / 24.0) as i64;
                    if day > 0 {
                        stat_t.uptime_str = format!("{} 天", day);
                    } else {
                        stat_t.uptime_str = format!(
                            "{:02}:{:02}:{:02}",
                            (stat_t.uptime as f64 / 3600.0) as i64,
                            (stat_t.uptime as f64 / 60.0) as i64 % 60,
                            stat_t.uptime % 60
                        );
                    }

                    info!("update stat `{:?}", stat_t);
                    {
                        let mut host_stat_map = stat_dict_1.lock().unwrap();
                        if let Some(pre_stat) = host_stat_map.get(&info.name) {
                            if pre_stat.latest_ts + OFFLINE_THRESHOLD < stat_t.latest_ts {
                                // node up notify
                                notifier_tx_1.send((Event::NodeUp, stat_c.to_owned()));
                            }
                        }
                        host_stat_map.insert(String::from(&info.name), stat_c);
                        //trace!("{:?}", host_stat_map);
                    }
                } else {
                    error!("invalid stat `{:?}", stat);
                }
            }
        });

        // timer thread
        let resp_json = self.resp_json.clone();
        let stat_dict_2 = stat_dict.clone();
        let notifier_tx_2 = notifier_tx.clone();
        let mut latest_notify_ts: u64 = 0;
        let mut latest_save_ts: u64 = 0;
        let notify_interval: u64 = cfg.notify_interval;
        thread::spawn(move || loop {
            let mut resp = StatsResp::new();
            let mut notified = false;
            {
                let mut host_stat_map = stat_dict_2.lock().unwrap();
                for (_, mut stat) in host_stat_map.iter_mut() {
                    if stat.disabled {
                        resp.servers.push(stat.to_owned().into_owned());
                        continue;
                    }
                    let mut stat_c = stat.borrow_mut();
                    let o = stat_c.to_mut();
                    // 10s 下线
                    if o.latest_ts + OFFLINE_THRESHOLD < resp.updated {
                        o.online4 = false;
                        o.online6 = false;
                    }

                    // notify check /30 s
                    if latest_notify_ts + notify_interval < resp.updated {
                        if o.online4 || o.online6 {
                            notifier_tx_2.send((Event::Custom, stat_c.to_owned()));
                        } else {
                            o.disabled = true;
                            notifier_tx_2.send((Event::NodeDown, stat_c.to_owned()));
                        }
                        notified = true;
                    }

                    resp.servers.push(stat_c.to_owned().into_owned());
                }
                resp.servers.sort_by(|a, b| a.pos.cmp(&b.pos));
                if notified {
                    latest_notify_ts = resp.updated;
                }
            }
            {
                let mut o = resp_json.lock().unwrap();
                *o = serde_json::to_string(&resp).unwrap();
            }
            // last_network_in/out save /60s
            if latest_save_ts + SAVE_INTERVAL < resp.updated {
                latest_save_ts = resp.updated;
                if (!resp.servers.is_empty()) {
                    if let Ok(mut file) = File::create("stats.json") {
                        file.write(serde_json::to_string(&resp).unwrap().as_bytes());
                        file.flush();
                        trace!("save stats.json succ!");
                    } else {
                        error!("save stats.json fail!");
                    }
                }
            }

            thread::sleep(Duration::from_millis(500))
        });

        // notify thread
        *NOTIFIER_HANDLE.lock().unwrap() = Some(Handle::current());
        let notifier_list = self.notifier_list.clone();
        thread::spawn(move || loop {
            while let Ok(msg) = notifier_rx.recv() {
                let (e, stat) = msg;
                let notifiers = &*notifier_list.lock().unwrap();
                trace!("recv notify => {}, {:?}, {:?}", notifiers.len(), e, stat);
                for notifier in notifiers {
                    notifier.do_notify(&e, stat.borrow());
                }
            }
        });

        Ok(())
    }

    pub fn get_stats_json(&self) -> String {
        String::from(self.resp_json.lock().unwrap().as_str())
    }

    pub fn report(&self, data: serde_json::Value) -> Result<()> {
        lazy_static! {
            static ref SENDER: SyncSender<Cow<'static, HostStat>> =
                STAT_SENDER.get().unwrap().clone();
        }

        match serde_json::from_value(data) {
            Ok(stat) => {
                trace!("send stat => {:?} ", stat);
                SENDER.send(Cow::Owned(stat));
            }
            Err(err) => {
                error!("report error => {:?}", err);
            }
        };
        Ok(())
    }
}
