import react from '@vitejs/plugin-react'
import { defineConfig } from 'vite'

// https://vite.dev/config/
export default defineConfig({
  plugins: [react()],
  // the Codama client reads process.env.NODE_ENV, which only exists in Node
  define: { "process.env": { NODE_ENV: "development" } },
})
