/** @type {import('jest').Config} */
module.exports = {
  preset: 'ts-jest',
  testEnvironment: 'node',
  testMatch: ['**/test/**/*.test.ts'],
  moduleNameMapper: {
    '^@common-creation/z85-wasm$': '<rootDir>/pkg-node/z85_wasm.js'
  },
  testTimeout: 10000,
  collectCoverageFrom: [
    'test/**/*.ts',
    '!test/**/*.d.ts'
  ]
};