import js from '@eslint/js';
import globals from 'globals';

export default [
  js.configs.recommended,
  {
    ignores: ['.prettierrc.cjs', 'dist/**/*'],
    languageOptions: {
      ecmaVersion: 'latest',
      sourceType: 'module',
      globals: {
        ...globals.browser,
        ...globals.webextensions,
      },
    },
    rules: {
      'no-unused-vars': 'off',
    },
  },
];
