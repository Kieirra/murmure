import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import tailwindcss from '@tailwindcss/vite';
import { tanstackRouter } from '@tanstack/router-plugin/vite';
import checker from 'vite-plugin-checker';
import { resolve } from 'path';

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;

// https://vite.dev/config/
export default defineConfig(async () => ({
    plugins: [
        tanstackRouter({
            target: 'react',
            autoCodeSplitting: true,
            enableRouteGeneration: false
        }),
        react(),
        tailwindcss(),
        checker({
            typescript: true,
        }),
    ],

    build: {
        rollupOptions: {
            input: {
                main: resolve(__dirname, 'index.html'),
                overlay: resolve(__dirname, 'src/overlay/index.html'),
            },
        },
    },

    resolve: {
        alias: {
            '@': '/src',
        },
    },

    // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
    //
    // 1. prevent Vite from obscuring rust errors
    clearScreen: false,
    // 2. tauri expects a fixed port, fail if that port is not available
    server: {
        port: 1420,
        strictPort: true,
        host: host || '127.0.0.1',
        hmr: {
            protocol: 'ws',
            host: host || '127.0.0.1',
            port: 1421,
        },
        watch: {
            // 3. tell Vite to ignore watching `src-tauri`
            ignored: ['**/src-tauri/**', '**/resources/**'],
        },
    },
}));
