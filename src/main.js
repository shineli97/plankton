/*
 * @Author: shineli shineli97@163.com
 * @Date: 2023-11-24 14:57:58
 * @LastEditors: shineli
 * @LastEditTime: 2023-11-28 10:45:10
 * @Description: file content
 */
import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";
import Antd from "ant-design-vue";
import "ant-design-vue/dist/reset.css";

const app = createApp(App);
app.use(Antd).mount("#app");
