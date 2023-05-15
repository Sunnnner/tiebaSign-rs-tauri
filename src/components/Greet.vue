<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { useMessage } from 'naive-ui'

const message = useMessage();
const loading = ref(false);
const input_value = ref("");

const blur = (e: any) => {
  input_value.value = e.target.value;
};

async function click() {
  message.info("正在签到");
  loading.value = true;
  invoke("sign_tb", {'bduss': input_value.value}).then((res: any) => {
      loading.value = false;
      message.success("签到成功");
  }).catch((err: any) => {
      loading.value = false;
      message.error(err.message)
  });

}


</script>

<template>
  <n-card  title="百度贴吧签到">
    <n-input-group>
      <n-input :style="{ width: '80%' }" placeholder="请输入bduss" :default-value="input_value" :on-blur="blur" />
      <n-button type="primary"  :loading="loading" @click="click">
        点击签到
      </n-button>
    </n-input-group>
  </n-card>
</template>

<style scoped>
</style>
