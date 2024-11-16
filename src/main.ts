import { createApp } from "vue";
import App from "./App.vue";
import { emit } from '@tauri-apps/api/event'

createApp(App).mount("#app");

declare global {
  interface Window {
      updateTitle: (title: string) => void;
      appsEmit: typeof emit;
  }
}

window.appsEmit = emit

window.addEventListener('load', () => {
  window.appsEmit('page-loaded')
})
