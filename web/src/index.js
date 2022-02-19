import Main from "./Main";
import {createApp} from "vue";
import {Grid, Layout, Progress, Table, Tag} from "ant-design-vue";
import { createVuetify } from 'vuetify';
import "vuetify/dist/vuetify.min.css";

const vuetify = createVuetify();

const app = createApp(Main)
.use(Layout)
.use(Grid)
.use(Table)
.use(Tag)
.use(vuetify)
.mount("#app");