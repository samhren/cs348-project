import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import tailwindcss from '@tailwindcss/vite'
import path from 'path'

export default defineConfig({
  plugins: [tailwindcss(), svelte()],
  server: {
    allowedHosts: [
      process.env.RAILWAY_PUBLIC_DOMAIN,
      '.up.railway.app',
    ].filter(Boolean) as string[],
  },
  resolve: {
    alias: {
      $lib: path.resolve('./src/lib'),
    },
  },
})
