import Main from "./Main";
import {createApp} from "vue";
import {Grid, Layout} from "ant-design-vue";

const app = createApp(Main)
.use(Layout)
.use(Grid)
.mount("#app");