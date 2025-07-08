# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a WebAssembly-based Z85 encoder/decoder library that provides efficient encoding and decoding of binary data using the Z85 format (ZeroMQ spec:32/Z85). The project uses Rust compiled to WebAssembly with TypeScript support.

## Common Development Commands

### Build Commands
- `npm run build` - Build WASM module for web target
- `npm run build:node` - Build WASM module for Node.js target
- `npm run build:debug` - Build WASM module in debug mode
- `npm run clean` - Clean build artifacts

### Test Commands
- `npm test` - Run Jest tests (builds Node version first)
- `npm run test:rust` - Run Rust tests with coverage (cargo llvm-cov)
- `npm run test:wasm` - Run WASM-specific tests (wasm-pack test)
- Running a single test: `npm test -- test/e2e.test.ts`

### Publishing
- `npm run prepublishOnly` - Runs before publishing (builds the package)
- Tagged releases trigger automatic npm publishing via GitHub Actions

## Architecture

### Core Structure
- **src/lib.rs** - Main Rust implementation with WASM bindings
  - Provides Z85/Base64 conversion functions
  - Supports Data URL format conversions
  - Handles automatic padding (Z85 requires data length divisible by 4)
  - Format: `{z85_data}:{padding_count}`

### Key Dependencies
- **wasm-bindgen** - Rust/WASM interop
- **z85** - Core Z85 encoding/decoding library
- **base64** - Base64 encoding/decoding
- **wasm-pack** - Build tool for WASM modules

### Build Outputs
- **pkg/** - Web target build output
- **pkg-node/** - Node.js target build output
- Both contain TypeScript definitions (.d.ts files)

## Testing Strategy
- Rust unit tests in src/lib.rs (comprehensive coverage)
- Jest E2E tests in test/e2e.test.ts for Node.js integration
- WASM-specific tests using wasm-bindgen-test