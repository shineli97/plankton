<!--
 * @Author: shineli shineli97@163.com
 * @Date: 2023-11-24 14:57:58
 * @LastEditors: shineli
 * @LastEditTime: 2023-11-27 16:46:19
 * @Description: file content
-->
<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const greetMsg = ref("");
const name = ref("");

async function greet() {
  greetMsg.value = await invoke("greet", { name: name.value });
}
</script>

<template>
  <form class="row" @submit.prevent="greet">
    <input id="greet-input" v-model="name" placeholder="Enter a name..." />
    <button type="submit">Greet</button>
  </form>

  <p>{{ greetMsg }}</p>
</template>
