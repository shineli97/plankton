<!--
 * @Author: shineli shineli97@163.com
 * @Date: 2023-12-13 15:53:34
 * @LastEditors: shineli
 * @LastEditTime: 2023-12-13 20:55:05
 * @Description: file content
-->
<template>
  <a-table :dataSource="dataList" :columns="columns">
    <template #bodyCell="{ text, record, column }">
      <template v-if="column.key === 'name'">
        <a @click="openFile(record)">{{ text }}</a>
      </template>
    </template>
  </a-table>
</template>

<script setup>
import { storeToRefs } from "pinia";
import { invoke } from "@tauri-apps/api";
import { useFileStore } from "@/store";

const store = useFileStore();
const { dataList } = storeToRefs(store);

const columns = [
  {
    title: "Name",
    dataIndex: "name",
    key: "name",
  },
  {
    title: "Path",
    dataIndex: "path",
    key: "path",
    ellipsis: true,
  },
  {
    title: "Size",
    dataIndex: "size",
    key: "size",
  },
  {
    title: "CreateDate",
    dataIndex: "created_date",
    key: "created_date",
  },
];

function openFile(record) {
  invoke("open", { path: record.path });
}
</script>
<style scoped>
.item {
  cursor: pointer;
}
.item:hover {
  color: #95d3ff;
}
.item:active {
  color: #c7c7c7;
}
</style>
