/*
 * @Author: shineli shineli97@163.com
 * @Date: 2023-12-01 15:18:36
 * @LastEditors: shineli
 * @LastEditTime: 2023-12-13 20:17:04
 * @Description: file content
 */
import { createRouter, createWebHashHistory } from "vue-router";
import Clipboard from "@/components/Clipboard.vue";
import FileList from "@/components/FileList.vue";
import Home from "@/views/home/index.vue";

export const menus = [
  {
    name: "clipboard",
    path: "/clipboard",
    component: Clipboard,
    meta: {
      name: "剪切板",
      color: "#55acee",
      show: true,
    },
  },
  {
    name: "fileList",
    path: "/fileList",
    component: FileList,
    meta: {
      name: "文件列表",
      color: "#55acee",
      show: false,
    },
  },
];

const routes = [
  {
    path: "/",
    redirect: "/home",
  },
  {
    name: "home",
    path: "/home",
    component: Home,
    children: [
      {
        name: "empty",
        path: "",
      },
    ].concat(menus),
  },
];

export const Router = createRouter({
  history: createWebHashHistory(),
  routes,
});
