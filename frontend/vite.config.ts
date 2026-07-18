import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import { fileURLToPath, URL } from 'node:url'

// Vite 配置：构建产物输出到 Tauri dist 目录，dev 模式代理 API 到 Axum 服务
export default defineConfig({
  plugins: [vue()],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url)),
    },
  },
  build: {
    outDir: '../rust/src-tauri/dist',
    emptyOutDir: true,
    // 单 exe 分发场景，适度分块但不过度拆分
    chunkSizeWarningLimit: 800,
    rollupOptions: {
      output: {
        manualChunks: {
          qrcode: ['qrcode-generator'],
        },
      },
    },
  },
  server: {
    port: 5173,
    strictPort: true,
    proxy: {
      // API 请求转发到 Axum 后端
      '/api': {
        target: 'http://127.0.0.1:5000',
        changeOrigin: true,
        // SSE 长连接支持
        ws: false,
        configure: (proxy) => {
          proxy.on('proxyReq', (proxyReq) => {
            proxyReq.setHeader('Connection', 'keep-alive')
          })
        },
      },
    },
  },
})
