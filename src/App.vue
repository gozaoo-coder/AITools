<script setup>
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import axios from 'axios'

const result = ref('')

async function callOnline() {
  const res = await axios.get('http://localhost:3030/api/hello')
  result.value = res.data
}

async function callLocal() {
  const res = await invoke('hello_local')
  result.value = JSON.parse(res)
}
</script>

<template>
    <MyButton @click="sayHi">本地组件</MyButton>
  <div>
    <button @click="callOnline">调用在线服务</button>
    <button @click="callLocal">调用本地服务</button>
    <pre>{{ result }}</pre>
  </div>
</template>