<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useMessage } from "naive-ui";
import type { DataTableColumns } from "naive-ui";

const message = useMessage();
const loading = ref(false);
const ali_loading = ref(false);
const input_value = ref("");
const ali_input_value = ref("");
const percentage_value = ref<string>("0")
type Song = {
  name: string;
  title: string;
};

const blur = (e: any) => {
  input_value.value = e.target.value;
};

const ali_blur = (e: any) => {
  ali_input_value.value = e.target.value;
};

async function click() {
  loading.value = true;
  let a: number = 0;
  let data_lenth: number = data.value.length;
  data.value.map((item: any) => {
    invoke("client_sign", { kw: item.name })
      .then((res: any) => {
        a += 1;
        percentage_value.value = ((a / data_lenth) * 100).toFixed(2);
        data.value.map((item: any) => {
          if (item.name == res) {
            item.title = "已签到";
          }
        });
      })
      .catch((err: any) => {
        console.log(err);
        message.error(err.message);
      });
  });
  loading.value = false;
  message.success("签到成功");
}


async function ali_click() {

}


const createColumns = ({
  play,
}: {
  play: (row: Song) => void;
}): DataTableColumns<Song> => {
  return [
    {
      title: "贴吧名称",
      key: "name",
    },
    {
      title: "签到状态",
      key: "title",
    },
  ];
};

const columns = createColumns({
  play: (row) => {
    console.log(row);
  },
});

const data = ref<any>([]);

const pagination = {
  pageSize: 5,
  showSizeChanger: false,
};

async function get_favorite_name() {
  invoke("get_favorite_name", { bduss: input_value.value })
    .then((res: any) => {
      res.map((item: any) => {
        // 将item放入data中
        data.value.push({
          name: item,
          title: "未签到",
        });
      });
    })
    .catch((err: any) => {
      message.error(err.message);
    });
}

onMounted(() => {
  invoke("get_bduss_os")
    .then((res: any) => {
      input_value.value = res;
    })
    .catch((err: any) => {
      message.error("bduss不存在,请手动输入");
    });
  // get_favorite_name()
});
</script>

<template>
  <n-card title="百度贴吧签到助手">
    <n-space vertical size="large">
      <n-input-group>
        <n-input
          :style="{ width: '100%' }"
          placeholder="请输入bduss"
          :on-blur="blur"
          v-model:value="input_value"
          type="password"
        />
        <n-button
          dashed
          type="primary"
          :loading="loading"
          @click="get_favorite_name"
          v-if="input_value"
        >
          获取贴吧名称
        </n-button>
        <n-button
          type="primary"
          :loading="loading"
          @click="click"
          v-if="data.length > 0"
        >
          点击签到
        </n-button>
      </n-input-group>

      <n-progress
        v-if="data.length > 0"
        type="line"
        :percentage=percentage_value
        :indicator-placement="'inside'"
        status="success"
        processing
      />
      <n-data-table
        :columns="columns"
        :data="data"
        :pagination="pagination"
        :bordered="false"
      />
    </n-space>
  </n-card>
  <n-card title="阿里云盘签到">
      <n-input-group>
          <n-input
                  :style="{ width: '100%' }"
                  placeholder="请输入token"
                  :on-blur="ali_blur"
                  v-model:value="ali_input_value"
                  type="password"
          />
          <n-button
                  type="primary"
                  :loading="ali_loading"
                  @click="ali_click"
                  v-if="data.length > 0"
          >
              点击签到
          </n-button>
      </n-input-group>
  </n-card>
</template>

<style scoped></style>
