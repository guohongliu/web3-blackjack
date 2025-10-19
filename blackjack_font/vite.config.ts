import vue from '@vitejs/plugin-vue'
import tailwindcss from '@tailwindcss/vite'
import { defineConfig } from 'vite'


// https://vitejs.dev/config/
export default defineConfig({
  plugins: [vue(),
    tailwindcss(),
  ],
  server: {
    proxy: {
      "/api": {
        target: "http://localhost:8080",
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/api/, "/api")
      }
    }
  },
  resolve: {
    alias: {
      '@': '/src',
    },
  },
  build: {
    sourcemap: true  // 启用源码映射，方便调试
  }
})
