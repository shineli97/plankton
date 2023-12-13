<!--
 * @Author: shineli shineli97@163.com
 * @Date: 2023-12-01 11:57:23
 * @LastEditors: shineli
 * @LastEditTime: 2023-12-13 20:55:40
 * @Description: file content
-->
<template>
  <div>
    <a-input
      v-model:value="value"
      :bordered="false"
      placeholder="Borderless"
      @pressEnter="handleEnter"
    />

    <Menu ref="menuRef" />
    <router-view />
  </div>
</template>

<script setup>
import Menu from "@/components/Menu.vue";
import { ref } from "vue";
import { useRouter, useRoute } from "vue-router";
import { useFileStore } from "@/store";
import { invoke } from "@tauri-apps/api/tauri";
import { WindowManager, LogicalSize } from "@tauri-apps/api/window";

const store = useFileStore();
const window = new WindowManager();
const router = useRouter();
const route = useRoute();
const value = ref("");
const menuRef = ref();

async function handleEnter() {
  const dataList = await invoke("search", { name: value.value });
  store.setDataList(dataList);
  window.setSize(new LogicalSize(600, 400));
  if (route.name !== "fileList") {
    router.push({ name: "fileList" });
  }
}
</script>
