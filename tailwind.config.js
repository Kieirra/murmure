module.exports = {
    content: [
        './index.html',
        './src/**/*.{js,ts,jsx,tsx}',
        '!./resources/**',
        '!./src-tauri/**',
        '!./node_modules',
    ],
    darkMode: 'class',
    plugins: [require('tailwindcss-animate')],
};
