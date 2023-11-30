<!--
 * @Author: shineli shineli97@163.com
 * @Date: 2023-11-28 10:40:37
 * @LastEditors: shineli
 * @LastEditTime: 2023-11-30 16:07:22
 * @Description: file content
-->
<template>
  <a-list size="small" bordered :data-source="dataList">
    <template #renderItem="{ item }">
      <a-list-item class="item">
        <div class="item_date">{{ item.data }}</div>
        <div class="item_end">
          <div class="item_end_action">
            <a @click="handleCopy(item)"><CopyOutlined /></a>
            <div class="item_end_action_split" />
            <a class="item_end_action_del" @click="handleDelete(item.id)"
              ><DeleteOutlined
            /></a>
          </div>
          <div class="item_end_time">{{ formatTime(item.time) }}</div>
        </div>
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

const formatTime = (time) => {
  let diff = new Date().getTime() - new Date(time).getTime();

  let min = Math.floor(diff / (60 * 1000));

  if (min < 60) {
    return `${min} 分钟前`;
  }
  let hour = Math.floor(min / 60);
  if (hour < 24) {
    return `${hour} 小时前`;
  }
  let day = Math.floor(hour / 24);
  return `${day} 天前`;
};

async function getData() {
  data.value = await invoke("get_one");
}

async function getDataList() {
  dataList.value = await invoke("get_all");
}

async function handleCopy(item) {
  const res = await invoke("copy", { id: item.id, data: item.data });
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

<style scoped>
.item {
  .item_date {
    word-break: break-all;
  }
  .item_end {
    .item_end_action {
      margin-left: 12px;
      display: flex;
      align-items: center;
      .item_end_action_split {
        width: 1px;
        height: 14px;
        margin: 0 2px;
        background-color: rgba(5, 5, 5, 0.06);
      }
      .item_end_action_del {
        color: #e94848;
      }
    }
    .item_end_time {
      font-size: 8px;
      float: right;
    }
  }
}
</style>
