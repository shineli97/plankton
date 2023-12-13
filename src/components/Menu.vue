<!--
 * @Author: shineli shineli97@163.com
 * @Date: 2023-12-01 17:04:11
 * @LastEditors: shineli
 * @LastEditTime: 2023-12-13 20:20:31
 * @Description: file content
-->
<template>
  <div style="height: 48px; width: 100%; display: flex">
    <template v-for="item in menus">
      <div class="menu" v-if="item.meta.show">
        <div style="margin: 0 8px" @click="handleClick">
          {{ item.meta.name }}
        </div>
      </div>
    </template>
  </div>
</template>

<script setup>
import { menus } from "@/router";
import { useRouter } from "vue-router";
import { WindowManager, LogicalSize } from "@tauri-apps/api/window";
const window = new WindowManager();

const router = useRouter();

function handleClick() {
  router.push({ name: "clipboard" });
  window.setSize(new LogicalSize(600, 400));
}

function handleSearch(val) {
  menus.filter();
}
</script>

<style scoped>
.menu {
  margin: 8px;
  height: 32px;
  line-height: 32px;
  background-color: rgba(240, 255, 255, 0.5);
  border-style: solid;
  border-width: thin;
  border-radius: 8px;
  box-shadow: rgb(137 137 137 / 60%) -3px 3px 3px;
  cursor: pointer;
}
.menu:hover {
  background-color: rgb(98, 165, 165);
}
</style>
