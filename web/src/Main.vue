<template>
<a-layout style="min-height: 100%;">
    <a-layout-header :class="['header']">
        Z
    </a-layout-header>

    <a-layout-content :class="['content']">

        <a-table 
            :class="['table']"
            rowKey="name" 
            :columns="columns" 
            :data-source="stats['servers']"
            :pagination="false"
            rowClassName="row"
        >
            <template #bodyCell="{column, record}">
                <template v-if="column.key === 'lastNetwork'">
                    {{formatBytes(record['last_network_in'])}}
                    /
                    {{formatBytes(record['last_network_out'])}}
                </template>

                <template v-if="column.key === 'load'">
                    {{record['load_1']}}/{{record['load_5']}}/{{record['load_15']}}
                </template>

                <template v-if="column.key === 'networkRealTime'">
                    <div :class="['network-realtime']">
                        <span>
                            <a-tag color="orange">
                                ↓ {{formatBytes(record['network_rx'])}}/s
                            </a-tag>
                        </span>
                        <span>-</span>
                        <span>
                            <a-tag color="green">
                                ↑ {{formatBytes(record['network_tx'])}}/s
                            </a-tag>
                        </span>
                    </div>
                </template>

                <template v-if="column.key === 'network'">
                    {{formatBytes(record['network_in'])}}
                    /
                    {{formatBytes(record['network_out'])}}
                </template>

                <template v-if="column.key === 'cpu'">
                    <div :class="['progress']">
                        <v-progress-linear
                            v-model="record['cpu']"
                            height="20"
                            rounded-bar
                            striped
                            :color="getColor(column.key, record['cpu'])"
                        >
                            <template #default="{value}">
                                <strong style="color: #787878;">{{value}}%</strong>
                            </template>
                        </v-progress-linear>
                    </div>
                </template>

                <template v-if="column.key === 'memory'">
                    <div :class="['progress']">
                        <v-progress-linear
                            :modelValue="getMemoryPercentage(record)"
                            height="20"
                            rounded-bar
                            striped
                            :color="getColor(column.key, getMemoryPercentage(record))"
                        >
                            <template #default="{value}">
                                <strong style="color: #787878;">{{value}}%</strong>
                            </template>
                        </v-progress-linear>
                    </div>
                </template>

                <template v-if="column.key === 'hdd'">
                    <div :class="['progress']">
                        <v-progress-linear
                            :modelValue="getHddPercentage(record)"
                            height="20"
                            rounded-bar
                            striped
                            :color="getColor(column.key, getHddPercentage(record))"
                        >
                            <template #default="{value}">
                                <strong style="color: #787878;">{{value}}%</strong>
                            </template>
                        </v-progress-linear>
                    </div>
                </template>

                <template v-if="column.key === 'lossRate'">
                    <a-tag :color="getColor(column.key, record['ping_189'])">{{record['ping_189']}}</a-tag> -
                    <a-tag :color="getColor(column.key, record['ping_10010'])">{{record['ping_10010']}}</a-tag> -
                    <a-tag :color="getColor(column.key, record['ping_10086'])">{{record['ping_10086']}}</a-tag>
                </template>
            </template>

            <template #expandedRowRender="{ record }" class="fdsafdsa">
                <table class="specific-info-table">
                    <tbody>
                        <tr>
                            <td>内存：</td>
                            <td>{{formatBytes(record['memory_used'] * 1024)}} / {{formatBytes(record['memory_total'] * 1024)}}</td>
                        </tr>
                        <tr>
                            <td>交换分区：</td>
                            <td>{{formatBytes(record['swap_used'])}} / {{formatBytes(record['swap_total'])}}</td>
                        </tr>
                        <tr>
                            <td>硬盘：</td>
                            <td>{{formatBytes(record['hdd_used'])}} / {{formatBytes(record['hdd_total'])}}</td>
                        </tr>
                        <tr>
                            <td>TCP/UDP/进/线：</td>
                            <td>{{record['tcp_count']}} / {{record['udp_count']}} / {{record['process_count']}} / {{record['thread_count']}}</td>
                        </tr>
                        <tr>
                            <td>联通/电信/移动：</td>
                            <td>{{record['time_189']}} / {{record['time_10010']}} / {{record['time_10086']}}</td>
                        </tr>
                    </tbody>
                </table>
            </template>
        </a-table>

    </a-layout-content>

    <a-layout-footer>
    </a-layout-footer>
</a-layout>
</template>

<script>
export default {
    data() {
        return {
            timer: null,
            stats: {"updated":0,"servers":[]},
            columns: [
            {
                title: "协议",
                key: "protocal",
                customRender: () => {
                    return 'IPV4';
                }
            }, {
                title: "月流量 ↓|↑",
                key: "lastNetwork" ,
                customCell: ({record, index, column}) => {
                    return {
                        class: "last-network"
                    };
                }
            }, {
                title: "节点名",
                dataIndex: 'alias'
            }, {
                title: "虚拟化",
                dataIndex: 'type'
            }, {
                title: "位置",
                dataIndex: 'location'
            }, {
                title: "在线时间",
                dataIndex: 'uptime'
            }, {
                title: "负载",
                key: 'load',
                responsive: ['sm']
            }, {
                title: "网络",
                key: "networkRealTime"
            }, {
                title: "总流量 ↓|↑",
                key: "network"
            }, {
                title: "处理器",
                key: "cpu"
            }, {
                title: "内存",
                key: "memory"
            }, {
                title: "硬盘",
                key: "hdd"
            }, {
                title: "丢包率(CU|CT|CM)",
                key: "lossRate"
            }]
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
        },
        getMemoryPercentage(record) {
            return (record['memory_used'].toFixed(2) / record['memory_total'] * 100).toFixed(2)
        },
        getHddPercentage(record) {
            return (record['hdd_used'] / record['hdd_total'] * 100).toFixed(2)
        },
        getColor(key, percentage) {
            switch (key) {
                case "hdd":
                    return percentage > 80 ? 'red' : 'green';
                case "memory":
                    return percentage > 80 ? 'red' : 'green';
                case "cpu":
                    return percentage > 50 ? 'red' : 'green';
                case "lossRate":
                    if(percentage < 10) return 'green';
                    if(percentage < 30) return 'orange';
                    return 'red';
            }

            return 'black';
        }
    },
    mounted() {
        this.timer = setInterval(() => {
            fetch('/json/stats.json')
                .then(res => res.json())
                .then(json => this.stats = json);
        }, 2000);
    }
}
</script>

<style lang="less">
.header {
    position: fixed;
    z-index: 1;
    width: 100%;
    color: white;
    font-size: 50px;
}
.content {
    display: flex;
    justify-content: center;
    margin-top: 64px + 20px;
    min-height: 100%;
}

.table {
    .ant-table-content > table {
        border-collapse: collapse;
        box-shadow: 0 0 20px 5px #d5d5d5;
        border-radius: 10px;
        border-spacing: 0;
        overflow: hidden;

        td, th {
            text-align: center;
        }

        .specific-info-table {
            display:flex;
            justify-content: center;
            
            > tbody {
                display: flex;
                flex-direction: column;
                align-items: center;
                width: 50%;

                tr {
                    display: flex;
                    width: 100%;
                }

                td {
                    &:first-child {
                        text-align: right;
                    }
                    &:last-child {
                        text-align: left;
                    }

                    flex: 1;
                    white-space: nowrap;
                }
            }
        }
    }
}

.last-network {

}
.network-realtime {
    display: flex;
    > span {
        &:first-child, &:last-child {
            flex: 1;

            > span {
                display: block;
                width: 100%;
            }
        }
        &:nth-child(2) {
            width: 10px;
        }
    }
}

.row {

}
.progress {
    width: 100px;
}
</style>