import { createApp } from 'vue'
import App from './App.vue'
import { invoke } from '@tauri-apps/api/tauri'
//const invoke = window.__TAURI__.invoke
//
// invoke('my_custom_command', { invokeMessage: 'Hello!' })

invoke('check_legal_moves')

createApp(App).mount('#app')
//createApp(ChessBoard).mount('#app')
