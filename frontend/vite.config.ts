import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  define: {
    // Required for Stellar SDK in browser
    global: 'globalThis',
  },
  resolve: {
    alias: {
      // Node polyfill for Stellar SDK
      buffer: 'buffer',
    },
  },
  optimizeDeps: {
    include: ['buffer'],
  },
})
