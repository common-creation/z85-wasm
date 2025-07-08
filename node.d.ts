// TypeScript definitions for Node.js wrapper
export * from './pkg-node/z85_wasm';

// Default export for init function (no-op in Node.js)
declare const init: () => Promise<void>;
export default init;