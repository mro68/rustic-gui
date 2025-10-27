import js from '@eslint/js';
import ts from '@typescript-eslint/eslint-plugin';
import tsParser from '@typescript-eslint/parser';
import svelte from 'eslint-plugin-svelte';
import svelteParser from 'svelte-eslint-parser';

export default [
    js.configs.recommended,
    {
        files: ['**/*.ts'],
        languageOptions: {
            parser: tsParser,
            parserOptions: {
                ecmaVersion: 2022,
                sourceType: 'module',
            },
        },
        plugins: {
            '@typescript-eslint': ts,
        },
        rules: {
            'prefer-const': 'error',
            '@typescript-eslint/no-unused-vars': ['error', { argsIgnorePattern: '^_' }],
            '@typescript-eslint/explicit-module-boundary-types': 'warn',
            '@typescript-eslint/no-explicit-any': 'error',
        },
    },
    {
        files: ['**/*.svelte'],
        languageOptions: {
            parser: svelteParser,
            parserOptions: {
                parser: tsParser,
            },
            globals: {
                console: 'readonly',
                Event: 'readonly',
                HTMLElement: 'readonly',
                HTMLInputElement: 'readonly',
                HTMLButtonElement: 'readonly',
                HTMLFormElement: 'readonly',
            },
        },
        plugins: {
            svelte,
        },
        rules: {
            ...svelte.configs.recommended.rules,
            'no-undef': 'warn',
        },
    },
    {
        ignores: ['node_modules/**', 'build/**', '.svelte-kit/**', 'src-tauri/**'],
    },
];