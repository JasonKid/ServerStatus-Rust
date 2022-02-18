<template>
<a-layout>
    <a-layout-header :class="['header']">
    </a-layout-header>
    <a-layout-content :class="['content']">
        <a-row :class="['row']">
            <a-col>协议</a-col>
            <a-col>月流量 ↓|↑</a-col>
            <a-col>节点名</a-col>
            <a-col>虚拟化</a-col>
            <a-col>位置</a-col>
            <a-col>在线时间</a-col>
            <a-col>负载</a-col>
            <a-col>网络 ↓|↑</a-col>
            <a-col>总流量 ↓|↑</a-col>
            <a-col>处理器</a-col>
            <a-col>内存</a-col>
            <a-col>硬盘</a-col>
            <a-col>丢包率(CU|CT|CM)</a-col>
        </a-row>
        <a-row :class="['row']" v-for="item in stats['servers']">
            <a-col>ipv4</a-col>
            <a-col>
                <a-row :class="['split-row']">
                    <a-col>{{formatBytes(item['last_network_in'])}}</a-col>
                    <a-col :class="['slash']">/</a-col>
                    <a-col>{{formatBytes(item['last_network_out'])}}</a-col>
                </a-row>
            </a-col>
            <a-col>{{item['alias']}}</a-col>
            <a-col>{{item['type']}}</a-col>
            <a-col>{{item['location']}}</a-col>
            <a-col>{{item['uptime']}}</a-col>
            <a-col>负载</a-col>
            <a-col>
                <a-row :class="['split-row']">
                    <a-col>{{formatBytes(item['network_rx'])}}/s</a-col>
                    <a-col :class="['slash']">/</a-col>
                    <a-col>{{formatBytes(item['network_tx'])}}/s</a-col>
                </a-row>
            </a-col>
            <a-col>
                <a-row :class="['split-row']">
                    <a-col>{{formatBytes(item['network_in'])}}</a-col>
                    <a-col :class="['slash']">/</a-col>
                    <a-col>{{formatBytes(item['network_out'])}}</a-col>
                </a-row>
            </a-col>
            <a-col>{{item['cpu']}}%</a-col>
            <a-col>{{(item['memory_used'].toFixed(2) / item['memory_total'] * 100).toFixed(2)}}%</a-col>
            <a-col>{{(item['hdd_used'] / item['hdd_total'] * 100).toFixed(2)}}%</a-col>
            <a-col>{{item['ping_189']}} | {{item['ping_10010']}} | {{item['ping_10086']}}</a-col>
        </a-row>
    </a-layout-content>
    <a-layout-footer>
    </a-layout-footer>
</a-layout>
</template>

<script>
export default {
    data() {
        return {
            stats: {"updated":1645098262,"servers":[{"name":"op","alias":"🐯","type":"x86","location":"🇺🇸","online4":true,"online6":false,"uptime":"811 天","load_1":0.25,"load_5":0.37,"load_15":0.36,"ping_10010":0.0,"ping_189":0.0,"ping_10086":0.0,"time_10010":153.0,"time_189":129.0,"time_10086":172.0,"tcp_count":61,"udp_count":12,"process_count":98,"thread_count":163,"network_rx":12893,"network_tx":18022,"network_in":56287231388,"network_out":59375773427,"last_network_in":12863799866,"last_network_out":13274635286,"cpu":12.0,"memory_total":1022252,"memory_used":380196,"swap_total":0,"swap_used":0,"hdd_total":20080,"hdd_used":13728,"custom":"","latest_ts":1645098261},{"name":"ion","alias":"🐝","type":"x86","location":"🇺🇸","online4":true,"online6":false,"uptime":"263 天","load_1":2.37,"load_5":3.02,"load_15":3.06,"ping_10010":12.0,"ping_189":13.0,"ping_10086":10.0,"time_10010":158.0,"time_189":150.0,"time_10086":192.0,"tcp_count":433,"udp_count":0,"process_count":98,"thread_count":252,"network_rx":1447780,"network_tx":437992,"network_in":2393812781004,"network_out":869683810026,"last_network_in":544974105011,"last_network_out":192922817692,"cpu":79.0,"memory_total":1881836,"memory_used":839964,"swap_total":1048572,"swap_used":394852,"hdd_total":59402,"hdd_used":28280,"custom":"","latest_ts":1645098262},{"name":"oc-01","alias":"🐢","type":"x86","location":"🇺🇸","online4":true,"online6":false,"uptime":"50 天","load_1":0.29,"load_5":0.25,"load_15":0.18,"ping_10010":6.0,"ping_189":4.0,"ping_10086":0.0,"time_10010":200.0,"time_189":156.0,"time_10086":219.0,"tcp_count":7,"udp_count":0,"process_count":115,"thread_count":227,"network_rx":3079,"network_tx":3946,"network_in":9112746028,"network_out":10353967342,"last_network_in":2070883658,"last_network_out":2441578889,"cpu":4.0,"memory_total":1009376,"memory_used":350772,"swap_total":8388604,"swap_used":77312,"hdd_total":38969,"hdd_used":16828,"custom":"","latest_ts":1645098261},{"name":"oc-02","alias":"🐢","type":"x86","location":"🇺🇸","online4":true,"online6":false,"uptime":"50 天","load_1":0.11,"load_5":0.17,"load_15":0.15,"ping_10010":7.0,"ping_189":3.0,"ping_10086":1.0,"time_10010":200.0,"time_189":194.0,"time_10086":203.0,"tcp_count":10,"udp_count":0,"process_count":117,"thread_count":252,"network_rx":4856,"network_tx":3954,"network_in":9975429556,"network_out":10634097103,"last_network_in":2098526605,"last_network_out":2459462588,"cpu":5.0,"memory_total":1009376,"memory_used":444084,"swap_total":8388604,"swap_used":135680,"hdd_total":38969,"hdd_used":13728,"custom":"","latest_ts":1645098261},{"name":"oc-03","alias":"🐢","type":"arm","location":"🇺🇸","online4":true,"online6":false,"uptime":"20 天","load_1":0.31,"load_5":0.51,"load_15":0.58,"ping_10010":21.0,"ping_189":17.0,"ping_10086":6.0,"time_10010":181.0,"time_189":170.0,"time_10086":195.0,"tcp_count":669,"udp_count":0,"process_count":207,"thread_count":515,"network_rx":671175,"network_tx":238562,"network_in":1657892607347,"network_out":514357793712,"last_network_in":377927519376,"last_network_out":112824945502,"cpu":9.0,"memory_total":24445812,"memory_used":16489552,"swap_total":0,"swap_used":0,"hdd_total":46054,"hdd_used":28393,"custom":"","latest_ts":1645098262},{"name":"ss1","alias":"🐹","type":"x86","location":"🇺🇸","online4":true,"online6":false,"uptime":"234 天","load_1":0.07,"load_5":0.28,"load_15":0.31,"ping_10010":64.0,"ping_189":4.0,"ping_10086":82.0,"time_10010":210.0,"time_189":144.0,"time_10086":1000.0,"tcp_count":21,"udp_count":0,"process_count":120,"thread_count":455,"network_rx":8491,"network_tx":2816,"network_in":49300416677,"network_out":62352533378,"last_network_in":11246825276,"last_network_out":14159654091,"cpu":4.0,"memory_total":1014656,"memory_used":473228,"swap_total":1048572,"swap_used":299872,"hdd_total":276922,"hdd_used":26029,"custom":"","latest_ts":1645098262},{"name":"rn","alias":"🐤","type":"x86","location":"🇺🇸","online4":true,"online6":false,"uptime":"5 天","load_1":0.33,"load_5":0.33,"load_15":0.33,"ping_10010":0.0,"ping_189":4.0,"ping_10086":9.0,"time_10010":206.0,"time_189":177.0,"time_10086":243.0,"tcp_count":23,"udp_count":0,"process_count":93,"thread_count":234,"network_rx":25927,"network_tx":30633,"network_in":49968452248,"network_out":14864448274,"last_network_in":11102784990,"last_network_out":3012501332,"cpu":10.0,"memory_total":1881836,"memory_used":395972,"swap_total":2097148,"swap_used":29136,"hdd_total":33230,"hdd_used":15339,"custom":"","latest_ts":1645098261},{"name":"txc","alias":"🐧","type":"x86","location":"🇨🇳","online4":true,"online6":false,"uptime":"17 天","load_1":0.0,"load_5":0.0,"load_15":0.0,"ping_10010":0.0,"ping_189":0.0,"ping_10086":0.0,"time_10010":32.0,"time_189":28.0,"time_10086":15.0,"tcp_count":5,"udp_count":0,"process_count":121,"thread_count":314,"network_rx":2861,"network_tx":5807,"network_in":30602883803,"network_out":3010533741,"last_network_in":7750091357,"last_network_out":604728233,"cpu":3.0,"memory_total":4030008,"memory_used":740460,"swap_total":0,"swap_used":0,"hdd_total":80579,"hdd_used":53320,"custom":"","latest_ts":1645098262},{"name":"pve","alias":"🐡","type":"x86","location":"🏠","online4":true,"online6":false,"uptime":"51 天","load_1":1.13,"load_5":0.66,"load_15":0.4,"ping_10010":6.0,"ping_189":0.0,"ping_10086":0.0,"time_10010":35.0,"time_189":30.0,"time_10086":25.0,"tcp_count":5,"udp_count":0,"process_count":534,"thread_count":1694,"network_rx":8486,"network_tx":5964,"network_in":1020266956409,"network_out":984118414341,"last_network_in":0,"last_network_out":0,"cpu":2.0,"memory_total":32830116,"memory_used":20385568,"swap_total":8388604,"swap_used":8888,"hdd_total":77806437,"hdd_used":4492010,"custom":"","latest_ts":1645098261},{"name":"kai1","alias":"k1","type":"x86","location":"🇰🇷","online4":true,"online6":false,"uptime":"4 天","load_1":0.0,"load_5":0.01,"load_15":0.0,"ping_10010":0.0,"ping_189":5.0,"ping_10086":0.0,"time_10010":51.0,"time_189":42.0,"time_10086":68.0,"tcp_count":23,"udp_count":0,"process_count":130,"thread_count":209,"network_rx":2987,"network_tx":1956,"network_in":2665752599,"network_out":2447341763,"last_network_in":0,"last_network_out":0,"cpu":2.0,"memory_total":995472,"memory_used":202092,"swap_total":0,"swap_used":0,"hdd_total":46051,"hdd_used":3273,"custom":"","latest_ts":1645098262},{"name":"kai2","alias":"k2","type":"x86","location":"🇰🇷","online4":true,"online6":false,"uptime":"4 天","load_1":0.01,"load_5":0.02,"load_15":0.0,"ping_10010":0.0,"ping_189":6.0,"ping_10086":0.0,"time_10010":48.0,"time_189":43.0,"time_10086":71.0,"tcp_count":1,"udp_count":0,"process_count":116,"thread_count":167,"network_rx":2652,"network_tx":972,"network_in":1222396213,"network_out":640910161,"last_network_in":0,"last_network_out":0,"cpu":2.0,"memory_total":990396,"memory_used":183016,"swap_total":0,"swap_used":0,"hdd_total":46051,"hdd_used":2799,"custom":"","latest_ts":1645098262}]},
        }
    },
    methods: {
        formatBytes(bytes, decimals = 2) {
            if (bytes === 0) return '0 Bytes';

            const k = 1024;
            const dm = decimals < 0 ? 0 : decimals;
            const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'];

            const i = Math.floor(Math.log(bytes) / Math.log(k));

            return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + ' ' + sizes[i];
        }
    }
}
</script>

<style scoped lang="less">
.header {
    position: fixed;
    z-index: 1;
    width: 100%;
}
.content {
    display: flex;
    align-items: center;
    margin-top: 64px + 20px;
    flex-direction: column;
}
.row {
    display: flex;
    width: 80%;

    > a-col {
        flex: 1 0 0px;
        white-space: nowrap;
        overflow: hidden;
        text-align: center;
    }
}
.split-row {
        display: flex;
        justify-content: space-between;

        > a-col {
            flex: 1 0 0px;
            text-align: right;
        }

        .slash {
            max-width: 20px;
            text-align: right;
        }
    }
</style>