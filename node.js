// Node.js compatibility wrapper
const wasmModule = require('./pkg-node/z85_wasm.js');

// In Node.js, init is a no-op since the WASM is loaded synchronously
const init = async () => {
  // No-op for Node.js compatibility
  return Promise.resolve();
};

// Export all functions
const {
  z85_to_base64,
  base64_to_z85,
  encode_z85,
  decode_z85,
  z85_to_base64_with_options,
  base64_to_z85_with_options,
  get_encoding_efficiency,
  init_wasm,
  ConversionOptions,
  DataType
} = wasmModule;

// CommonJS exports
module.exports = {
  default: init,
  z85_to_base64,
  base64_to_z85,
  encode_z85,
  decode_z85,
  z85_to_base64_with_options,
  base64_to_z85_with_options,
  get_encoding_efficiency,
  init_wasm,
  ConversionOptions,
  DataType
};

// ESM compatibility
module.exports.default = init;