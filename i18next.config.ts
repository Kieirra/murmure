import { defineConfig } from 'i18next-cli';

export default defineConfig({
    locales: ['fr'],
    extract: {
        input: ['src/**/*.{js,jsx,ts,tsx}'],
        output: 'src/i18n/locales/{{language}}.json',
        defaultNS: false,
        keySeparator: false,
        nsSeparator: false,
    },
});
