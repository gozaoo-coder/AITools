import { createApp } from 'vue'
import './style.css'
import App from './App.vue'
import MyUiLib from 'my-ui-lib'
import 'my-ui-lib/dist/style.css'   // 如有全局样式

createApp(App).use(MyUiLib).mount('#app')
