/*
 * @Author: shineli shineli97@163.com
 * @Date: 2023-12-13 20:43:47
 * @LastEditors: shineli
 * @LastEditTime: 2023-12-13 20:55:28
 * @Description: file content
 */
import { defineStore } from "pinia";

export const useFileStore = defineStore("counter", {
  state: () => ({ files: [] }),
  getters: {
    dataList: (state) => state.files,
  },
  actions: {
    setDataList(val) {
      this.files = val;
    },
  },
});
