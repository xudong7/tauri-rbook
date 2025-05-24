import { createApp } from "vue";
import App from "./App.vue";
import ElementPlus from "element-plus";
import "element-plus/dist/index.css";
import zhCn from "element-plus/es/locale/lang/zh-cn";
import router from './router';
import "./assets/global.css";
import { themeManager } from "./utils/themeManager";

const app = createApp(App);
app.use(ElementPlus, {
  locale: zhCn,
});
app.use(router);

// 初始化主题
themeManager.initializeTheme();

app.mount("#app");
