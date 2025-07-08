# @common-creation/z85-wasm

A WebAssembly-based Z85 encoder/decoder library that provides efficient encoding and decoding of binary data using the Z85 format (ZeroMQ spec:32/Z85).

## Features

- ✅ Full Z85 encoding and decoding support
- ✅ Automatic padding handling (Z85 requires data length divisible by 4)
- ✅ Base64 ↔ Z85 conversion utilities
- ✅ TypeScript support with full type definitions
- ✅ Optimized WASM implementation for performance
- ✅ ~6.67% bandwidth savings compared to Base64

## Installation

Install from npm:

```bash
npm install @common-creation/z85-wasm
```

Or install from GitHub (development version):

```bash
npm install github:common-creation/z85-wasm
```

## Usage

### Basic Example

```javascript
import init, { z85_to_base64, base64_to_z85, encode_z85, decode_z85 } from '@common-creation/z85-wasm';

// Initialize the WASM module (required before using any functions)
await init();

// Convert between Base64 and Z85
const base64Data = "SGVsbG8gV29ybGQ="; // "Hello World" in base64
const z85Data = await base64_to_z85(base64Data);
console.log(z85Data); // Output: "87cURD_*#TDfAS:0"

// Convert back
const base64Again = await z85_to_base64(z85Data);
console.log(base64Again); // Output: "SGVsbG8gV29ybGQ="

// Encode raw bytes
const rawData = new TextEncoder().encode("Hello, World!");
const encoded = await encode_z85(rawData);
console.log(encoded); // Z85 encoded with padding info

// Decode back to raw bytes
const decoded = await decode_z85(encoded);
console.log(new TextDecoder().decode(decoded)); // Output: "Hello, World!"
```

### TypeScript Example

```typescript
import init, { 
  z85_to_base64, 
  base64_to_z85, 
  encode_z85, 
  decode_z85,
  get_encoding_efficiency 
} from '@common-creation/z85-wasm';

async function example() {
  await init();
  
  // Get encoding efficiency stats
  const stats = get_encoding_efficiency(100000); // 100KB
  console.log(stats);
  // {
  //   original_size: 100000,
  //   base64_size: 133336,
  //   z85_size: 125000,
  //   efficiency_ratio: 0.9375,
  //   bandwidth_saving: 6.25
  // }
}
```

## API Reference

### Core Types

```typescript
enum DataType {
  Raw,      // Raw data format (default)
  DataURL   // Data URL format (e.g., data:image/png;base64,...)
}

class ConversionOptions {
  constructor(input: DataType, output: DataType);
  input: DataType;
  output: DataType;
}
```

### Functions

#### `init(): Promise<void>`
Initialize the WASM module. Must be called before using any other functions. This is the default export that initializes the WebAssembly module asynchronously.

#### `initSync(): void`
Alternative synchronous initialization method. Use this when you need to initialize the module synchronously (e.g., in certain bundler configurations).

#### `init_wasm(): void`
Internal initialization function that's automatically called when the WASM module loads. You typically don't need to call this directly - use `init()` instead.

#### `z85_to_base64(z85_data_with_padding: string): string`
Convert Z85 encoded data (with padding info) to Base64.

#### `z85_to_base64_with_options(data: string, options?: ConversionOptions): string`
Convert Z85 to Base64 with format options. Supports Data URL conversion.

```javascript
// Data URL to Data URL
const result = await z85_to_base64_with_options(
  "data:image/png;z85,encoded:0",
  new ConversionOptions(DataType.DataURL, DataType.DataURL)
);
// → "data:image/png;base64,..."

// Data URL to Raw
const result = await z85_to_base64_with_options(
  "data:image/jpeg;z85,encoded:0",
  new ConversionOptions(DataType.DataURL, DataType.Raw)
);
// → "base64data"
```

#### `base64_to_z85(base64_data: string): string`
Convert Base64 encoded data to Z85 with padding info.

#### `base64_to_z85_with_options(data: string, options?: ConversionOptions): string`
Convert Base64 to Z85 with format options. Supports Data URL conversion.

```javascript
// Data URL to Data URL
const result = await base64_to_z85_with_options(
  "data:image/png;base64,iVBORw0...",
  new ConversionOptions(DataType.DataURL, DataType.DataURL)
);
// → "data:image/png;z85,..."
```

#### `encode_z85(data: Uint8Array): string`
Encode raw bytes to Z85 format with padding info.

#### `decode_z85(z85_data_with_padding: string): Uint8Array`
Decode Z85 data (with padding info) to raw bytes.

#### `get_encoding_efficiency(original_size: number): object`
Calculate encoding efficiency comparison between Base64 and Z85.

## Z85 Format

This library uses the Z85 format as specified in [ZeroMQ RFC 32](https://rfc.zeromq.org/spec/32/). The implementation includes automatic padding handling:

- Z85 requires input length to be divisible by 4
- Padding bytes are automatically added during encoding
- Padding information is preserved in the format: `{z85_data}:{padding_count}`
- Padding is automatically removed during decoding

## Development

### Building from Source

```bash
# Clone the repository
git clone https://github.com/common-creation/z85-wasm.git
cd z85-wasm

# Install dependencies
npm install

# Build WASM module
npm run build

# Run tests
npm test
```

### Publishing

The package is automatically published to npm when a new tag is pushed:

```bash
git tag v1.0.1
git push origin v1.0.1
```

### Setting up npm Token

To enable automated publishing, add your npm token as a GitHub secret:

1. Get your npm token: `npm token create`
2. Add it to GitHub: Settings → Secrets → New repository secret
3. Name: `NPM_TOKEN`
4. Value: Your npm token

## Requirements

- Node.js >= 16.0.0
- Rust toolchain (for building from source)
- wasm-pack (installed automatically during build)

## License

MIT

## Contributing

Issues and pull requests are welcome at the [GitHub repository](https://github.com/common-creation/z85-wasm).