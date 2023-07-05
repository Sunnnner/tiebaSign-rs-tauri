<template>
  <div class="box flex">
    <n-avatar :size="100" :src="dataForm.avatar" alt="头像"/>
    <div class="box-message flex">
      <n-alert class="alert" title="本站公告" type="info" :closable="false"
                description="此页面为呓语API的测试页面，接口和页面不具有稳定性，随时存在变动。如果您想使用呓语API，请搭建自己的页面，切勿使用此页面作为嵌入页面。如有问题请联系站长。"
                show-icon/>
      <n-alert class="alert" title="目前支持 " type="success" :closable="false"
                description="抖音/快手/皮皮虾/最右"
                show-icon/>
      <n-alert class="alert" title="温馨提示" type="warning" :closable="false"
                description="粘贴视频地址时无需删除文案 但如果视频链接正确但解析失败请删掉文案后重试" show-icon/>
    </div>
    <n-input
        class="box-input"
        v-model="shortVideoUrl"
        placeholder="请粘贴分享链接"
        v-loading="flag.loading"
    >
      <template #append>
        <n-button @click="getDataForm">解析</n-button>
      </template>
    </n-input>
    <div class="box-videos flex" v-if="flag.type==='videos'">
      <div class="box-btn">
        <n-button @click="downloadFile(dataForm.video,'video')">下载视频</n-button>
        <n-button @click="downloadFile(dataForm.cover,'cover')" :disabled="dataForm.cover==='无'">下载封面</n-button>
        <n-button @click="downloadFile(dataForm.music,'music')" :disabled="dataForm.music==='无'">下载音乐</n-button>
      </div>
      <video class="video" controls width="600">
        <source :src="dataForm.video" type="">
      </video>
    </div>
    <div class="box-images flex" v-else-if="flag.type==='images'">
      <div class="btn-group">
        <n-select v-model="imageList" multiple collapse-tags placeholder="不选则默认下载全部">
          <n-option v-for="(item, index) in dataForm.images" :key="item.value" :label="index + 1"
                     :value="index"/>
        </n-select>
        <n-button style="margin: 20px 0" type="success" @click="downloadFile('','images')">下载</n-button>
      </div>
      <div class="images">
        <n-image style="height: 200px;margin: 5px" v-for="(item,index) in dataForm.images" :src="item" :alt="index"
                  lazy/>
      </div>
    </div>
    <n-empty v-else description="空空如也"/>
  </div>
</template>

<script setup lang="ts">
import {ref} from "vue";
import axios from "axios";
import { useMessage } from "naive-ui";
import {saveAs} from 'file-saver';


const message = useMessage();
// 定义变量
let shortVideoUrl = ref('')
let imageList = ref(null)
let fileBlob = ref(null)
let flag = ref({
  loading: false,
  type: ''
})
let dataForm = ref({
  title: '',
  author: '',
  avatar: 'https://cube.elemecdn.com/3/7c/3ea6beec64369c2642b92c6726f1epng.png',
  cover: '',
  music: '',
  video: '',
  images: null
})

async function getDataForm() {
  flag.value.loading = true
  await axios({
    method: 'get',
    url: 'https://api.zhaozeyu.top/v1/short-video',
    params: {url: shortVideoUrl.value}
  }).then(async res => {
    if (res.data.code === 0) {
      dataForm.value = res.data.data
      if (dataForm.value.video) {
        fileBlob.value = await getStream(dataForm.value.video)
        dataForm.value.video = window.URL.createObjectURL(fileBlob.value)
        flag.value.type = 'videos'
      } else {
        flag.value.type = 'images'
      }
    }
    message({
      type: res.data.code === 0 ? 'success' : 'error',
      message: res.data.message,
      showClose: true,
      grouping: true,
      center: true
    })
  }).catch(err => {
    console.log(err)
    message.error({
      message: '解析失败，请稍后再试',
      showClose: true,
      grouping: true,
      center: true
    })
  })
  flag.value.loading = false
}

async function downloadFile(url, type) {
  if (type === 'video') {
    saveAs(fileBlob.value, `${Date.now()}.mp4`)
  } else if (type === 'cover') {
    saveAs(dataForm.value.cover.replace(/http:/, 'https:'), `${Date.now()}.png`)
  } else if (type === 'music') {
    saveAs(dataForm.value.music, `${Date.now()}.mp3`)
  } else if (type === 'images') {
    if (imageList.value.length === 0) {
      for (let i = 0; i < dataForm.value.images.length; i++) {
        saveAs(dataForm.value.images[i].replace(/http:/, 'https:'), `${Date.now()}.png`)
      }
    } else {
      for (let i = 0; i < imageList.value.length; i++) {
        saveAs(dataForm.value.images[imageList.value[i]].replace(/http:/, 'https:'), `${Date.now()}.png`)
      }
      imageList.value = []
    }
  } else {
    message.error({
      message: '下载失败，请稍后再试',
      showClose: true,
      grouping: true,
      center: true
    })
  }
}

function getStream(url) {
  return axios({
    method: 'get',
    url: 'https://api.zhaozeyu.top/v1/download',
    params: {url: url},
    responseType: 'blob'
  }).then(res => {
    return res.data
  })
}
</script>

<style scoped>
.box {
  width: 100%;
  padding: 20px;
  border: 1px gray solid;
  border-radius: 20px;
}

.box-message {
  margin: 20px 0;
}

.alert {
  margin: 10px 0;
}

.box-input {
  margin-top: 20px;
  max-width: 1000px;
}

.video {
  max-width: 600px;
  max-height: 400px;
  border-radius: 20px;
}

.box-btn {
  margin: 20px;
  display: flex;
  justify-content: space-around;
}

.images {
  display: flex;
  align-items: center;
  justify-content: space-around;
  flex-wrap: wrap;
}

@media (max-width: 900px) {
  .video {
    width: 96%;
  }
}
</style>