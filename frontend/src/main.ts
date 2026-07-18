import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'

// 样式按原 index.html 顺序导入，保证优先级一致
import './styles/tokens.css'
import './styles/base.css'
import './styles/components.css'
import './styles/layout.css'
import './styles/files.css'
import './styles/modals.css'
import './styles/guided-tour.css'

const app = createApp(App)
app.use(createPinia())
app.mount('#app')
