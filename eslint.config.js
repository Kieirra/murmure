import js from '@eslint/js';
import tseslint from 'typescript-eslint';
import reactPlugin from 'eslint-plugin-react';
import reactHooks from 'eslint-plugin-react-hooks';
import pluginJest from 'eslint-plugin-jest';

export default [
    {
        ignores: [
            '**/node_modules/**',
            '**/dist/**',
            '**/src-tauri/**',
            '**/resources/**',
        ],
    },
    js.configs.recommended,
    ...tseslint.configs.recommended,
    {
        files: ['**/*.{ts,tsx}'],
        plugins: {
            react: reactPlugin,
            'react-hooks': reactHooks
        },
        languageOptions: {
            parser: tseslint.parser,
            parserOptions: {
                ecmaVersion: 'latest',
                sourceType: 'module',
                ecmaFeatures: {
                    jsx: true,
                },
                project: false,
            },
        },
        rules: {
            'react/react-in-jsx-scope': 'off',
            'react/prop-types': 'off',
            '@typescript-eslint/no-unused-vars': 'error',
            '@typescript-eslint/no-explicit-any': 'warn',
            ...reactHooks.configs.recommended.rules,
        },
        settings: {
            react: {
                version: 'detect',
            },
        },
    },
    {
        files: ['e2e-tests/**/*.js'],
        plugins: {
            jest: pluginJest
        },
        languageOptions: {
            globals: pluginJest.environments.globals.globals,
        }
    }
];
