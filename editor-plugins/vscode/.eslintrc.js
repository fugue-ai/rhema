module.exports = {
  root: true,
  parserOptions: {
    ecmaVersion: 2020,
    sourceType: 'module',
  },
  env: {
    node: true,
    es6: true,
  },
  extends: [
    'eslint:recommended',
  ],
  rules: {
    'no-unused-vars': 'warn',
    'no-undef': 'warn',
  },
  ignorePatterns: ['dist/', 'node_modules/', '*.js'],
}; 