import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import { resolve } from 'path';
// https://vitejs.dev/config/
export default defineConfig({
  plugins: [svelte({
    configFile: '../../svelte.config.js'
  })],
  root: resolve(__dirname, 'src/routes'),
  publicDir: resolve(__dirname, 'public'),
  build: {
    outDir: resolve(__dirname, 'dist'),
    emptyOutDir: true,
    sourcemap: true,
    rollupOptions: {
      input: {
        main: resolve(__dirname, 'src/routes/', 'index.html'),
        machines: resolve(__dirname, 'src/routes/machines/index.html'),
      },
    },
  },
})