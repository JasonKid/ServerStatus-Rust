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
            :expandRowByClick="true"
        >
            <template #bodyCell="{column, record}">
                <template v-if="column.key === 'lastNetwork'">
                    {{formatBytes(record['last_network_in'])}}
                    /
                    {{formatBytes(record['last_network_out'])}}
                </template>

                <template v-if="column.key === 'load'">
                    <span>
                        {{record['load_1']}}
                    </span>
                    <span>/</span>
                    <span>
                        {{record['load_5']}}
                    </span>
                    <span>/</span>
                    <span>
                        {{record['load_15']}}
                    </span>
                </template>

                <template v-if="column.key === 'networkRealTime'">
                    <div>
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
                                <strong style="color: #000000;">{{value}}%</strong>
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
                            :color="getColor(column.key, getMemoryPercentage(record))"
                        >
                            <template #default="{value}">
                                <strong style="color: #000000;">{{value}}%</strong>
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
                            :color="getColor(column.key, getHddPercentage(record))"
                        >
                            <template #default="{value}">
                                <strong style="color: #000000;">{{value}}%</strong>
                            </template>
                        </v-progress-linear>
                    </div>
                </template>

                <template v-if="column.key === 'lossRate'">
                    <div>
                        <a-tag :color="getColor(column.key, record['ping_189'])">{{record['ping_189']}}</a-tag>
                        <a-tag :color="getColor(column.key, record['ping_10010'])">{{record['ping_10010']}}</a-tag>
                        <a-tag :color="getColor(column.key, record['ping_10086'])">{{record['ping_10086']}}</a-tag>
                    </div>
                </template>
            </template>

            <template #expandedRowRender="{ record }">
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
                },
                customCell: () => ({class: "protocal"}), 
                customHeaderCell: () => ({class: "protocal"}), 
            }, {
                title: "节点",
                dataIndex: 'alias',
                customCell: () => ({class: "alias"}), 
                customHeaderCell: () => ({class: "alias"}), 
                fixed: true
            }, {
                title: "月流量 ↓|↑",
                key: "lastNetwork" ,
                customCell: () => ({class: "last-network"}), 
                customHeaderCell: () => ({class: "last-network"}), 
            }, {
                title: "虚拟化",
                dataIndex: 'type',
                customCell: () => ({class: "type"}), 
                customHeaderCell: () => ({class: "type"}), 
            }, {
                title: "位置",
                dataIndex: 'location',
                customCell: () => ({class: "location"}), 
                customHeaderCell: () => ({class: "location"}), 
            }, {
                title: "在线时间",
                dataIndex: 'uptime',
                customCell: () => ({class: "uptime"}), 
                customHeaderCell: () => ({class: "uptime"}), 
            }, {
                title: "负载",
                key: 'load',
                customCell: () => ({class: "load"}), 
                customHeaderCell: () => ({class: "load"}), 
            }, {
                title: "网络",
                key: "networkRealTime",
                customCell: () => ({class: "network-real-time"}), 
                customHeaderCell: () => ({class: "network-real-time"}), 
            }, {
                title: "总流量 ↓|↑",
                key: "network",
                customCell: () => ({class: "network"}), 
                customHeaderCell: () => ({class: "network"}), 
            }, {
                title: "处理器",
                key: "cpu",
                customCell: () => ({class: "cpu"}), 
                customHeaderCell: () => ({class: "cpu"}), 
            }, {
                title: "内存",
                key: "memory",
                customCell: () => ({class: "memory"}), 
                customHeaderCell: () => ({class: "memory"}), 
            }, {
                title: "硬盘",
                key: "hdd",
                customCell: () => ({class: "hdd"}), 
                customHeaderCell: () => ({class: "hdd"}), 
            }, {
                title: "丢包率(CU|CT|CM)",
                key: "lossRate",
                customCell: () => ({class: "loss-rate"}), 
                customHeaderCell: () => ({class: "loss-rate"}), 
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
                case "memory":
                case "hdd":
                case "cpu":
                    if(percentage >= 90) return 'red';
                    if(percentage >= 80) return 'orange';
                    return 'green';
                case "lossRate":
                    if(percentage >= 50) return 'red';
                    if(percentage >= 20) return 'orange';
                    return 'green';
            }

            return 'black';
        }
    },
    mounted() {
        fetch('/json/stats.json')
                .then(res => res.json())
                .then(json => this.stats = json);

        this.timer = setInterval(() => {
            fetch('/json/stats.json')
                .then(res => res.json())
                .then(json => this.stats = json);
        }, 2000);
    },
    unmounted() {
        clearInterval(this.timer);
    }
}
</script>

<style lang="less">
@xs: (max-width: 1650px);

body {
    overflow-x: hidden;
}

.header {
    position: fixed;
    z-index: 100;
    width: 100%;
    color: white;
    font-size: 50px;
}
.content {
    display: inline-flex;
    justify-content: center;
    margin-top: 64px + 20px;
    min-height: 100%;

    .ant-table-content {
        overflow: scroll;

        -ms-overflow-style: none;
        scrollbar-width: none;

        &::-webkit-scrollbar { 
            display: none;  /* Safari and Chrome */
        }
    }
}

.table {
    .ant-table-content > table {
        border-collapse: collapse;
        box-shadow: 0 0 20px 5px #d5d5d5;
        border-radius: 10px;
        border-spacing: 0;

        td, th {
            text-align: center;
            white-space: nowrap;
        }

        .ant-table-expand-icon-col {
            visibility: collapse;
        }

        td.network-real-time {
            > div {
                display: flex;
                > span {
                    &:first-child, &:last-child {
                        width: 100px;
                        overflow: hidden;

                        > span {
                            width: 100%;
                        }
                    }
                    &:nth-child(2) {
                        flex: 1;
                    }
                }
            }
        }

        td.loss-rate {
            > div {
                display: flex;
                justify-content: center;

                > span {
                    display: block;
                    width: 30px;
                    overflow: hidden;
                }
            }
        }

        .progress {
            display: flex;
            justify-content: center;
            width: 100px;

            .v-progress-linear__determinate {
                transition: all .5s cubic-bezier(.4,0,.2,1);
            }
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

@media only screen and @xs {
    .last-network, .protocal, .type, .uptime, .network, .location {
        display: none !important;
    }

    .table {
        .ant-table-expand-icon-col {
            width: auto !important;
        }

        th {
            font-size: 12px !important;
        }
        
        td, th {
            padding: 10px !important;
        }

        td.network-real-time {
            > div {
                flex-direction: column;
                > span {
                    margin: 3px 0px !important;
                    &:nth-child(2) {
                        display: none;
                    }
                    
                }
            }
        }

        td.load {
            > span {
                display: none;
                &:nth-child(3) {
                    display: inline;
                }
            }
        }

        .progress {
            width: 60px !important;
            font-size: 8px !important;
        }

        .specific-info-table {
            justify-content: flex-start !important;
        }
    }
}
</style>