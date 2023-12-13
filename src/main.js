/*
 * @Author: shineli shineli97@163.com
 * @Date: 2023-11-24 14:57:58
 * @LastEditors: shineli
 * @LastEditTime: 2023-12-01 15:26:26
 * @Description: file content
 */
import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";
import { createPinia } from "pinia";
import Antd from "ant-design-vue";
import "ant-design-vue/dist/reset.css";
import { Router } from "./router";

const pinia = createPinia();
const app = createApp(App);
app.use(Antd);
app.use(Router);
app.use(pinia);
app.mount("#app");
