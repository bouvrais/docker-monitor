import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import { resolve } from 'path';
// https://vitejs.dev/config/
export default defineConfig({
  plugins: [svelte()],
  root: resolve(__dirname, 'src/routes'),
  publicDir: resolve(__dirname, 'public'),
  build: {
    outDir: resolve(__dirname, 'dist'),
    emptyOutDir: true,
    rollupOptions: {
      input: {
        main: resolve(__dirname, 'src/routes/', 'index.html'),
        nested: resolve(__dirname, 'src/routes/nested/index.html'),
      },
    },
  },
})