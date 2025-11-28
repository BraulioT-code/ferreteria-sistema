import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

export default defineConfig({
  plugins: [react()],
  server: {
    port: 5173, // El puerto por defecto de Vite
    proxy: {
      // Si el frontend llama a /api/productos, se redirigirá a http://localhost:3000/api/productos
      '/api': {
        target: 'http://127.0.0.1:3000', // El puerto de tu servidor Rust (backend)
        changeOrigin: true,
        secure: false,
      },
    },
  },
});