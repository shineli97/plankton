<!--
 * @Author: shineli shineli97@163.com
 * @Date: 2023-11-28 10:40:37
 * @LastEditors: shineli
 * @LastEditTime: 2023-11-29 12:08:23
 * @Description: file content
-->
<template>
  <a-list size="small" bordered :data-source="dataList">
    <template #renderItem="{ item }">
      <a-list-item>
        <template #actions>
          <a @click="handleCopy(item.data)"><CopyOutlined /></a>
          <a style="color: #e94848" @click="handleDelete(item.id)"
            ><DeleteOutlined
          /></a>
        </template>
        <div>{{ item.data }}</div>
      </a-list-item>
    </template>
    <template #header>
      <div>{{ data }}</div>
    </template>
  </a-list>
</template>

<script setup>
import { ref, watchEffect } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { CopyOutlined, DeleteOutlined } from "@ant-design/icons-vue";
import { message } from "ant-design-vue";

const timer = ref("");
const dataList = ref([]);
const data = ref("");

watchEffect(() => {
  if (timer.value) {
    clearInterval(timer.value);
  }
  timer.value = setInterval(() => {
    getDataList();
    getData();
  }, 1000);
});

async function getData() {
  data.value = await invoke("get_one");
}

async function getDataList() {
  dataList.value = await invoke("get_all");
}

async function handleCopy(value) {
  const res = await invoke("copy", { value });
  if (res === "success") {
    message.success("copied!");
  } else {
    message.error("copy error");
  }
}

async function handleDelete(id) {
  const res = await invoke("delete", { id });
  if (res === 1) {
    message.success("deleted!");
  } else {
    message.info("delete error");
  }
}
</script>
