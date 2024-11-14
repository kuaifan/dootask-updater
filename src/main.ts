import { createApp } from "vue";
import App from "./App.vue";
import { emit } from '@tauri-apps/api/event'

createApp(App).mount("#app");

// 在页面加载完成后发送事件
window.addEventListener('load', () => {
  setTimeout(() => {
    emit('page-loaded')
  }, 300)
})
