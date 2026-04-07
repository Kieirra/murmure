import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import tailwindcss from '@tailwindcss/vite';
import { resolve } from 'node:path';

export default defineConfig({
    root: resolve(__dirname, 'src/smartmic'),
    plugins: [react(), tailwindcss()],
    build: {
        outDir: resolve(__dirname, 'src-tauri/resources/smartmic'),
        emptyOutDir: true,
        rollupOptions: {
            output: {
                entryFileNames: 'smartmic.js',
                assetFileNames: 'smartmic.[ext]',
            },
        },
    },
    resolve: {
        alias: {
            '@': resolve(__dirname, 'src'),
        },
    },
    base: './',
});
