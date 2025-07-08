import type * as WasmModule from '../pkg-node/z85_wasm';

describe('Z85 WASM E2E Tests', () => {
  let wasm: typeof WasmModule;

  beforeAll(async () => {
    // Import the built WASM module for Node.js
    wasm = await import('../pkg-node/z85_wasm.js');
  });

  describe('Basic encoding/decoding', () => {
    it('should convert Z85 to base64', () => {
      const result = wasm.z85_to_base64('nm=QNzY&b1A+]m^:1');
      expect(result).toBe('SGVsbG8gV29ybGQ=');
    });

    it('should convert base64 to Z85', () => {
      const result = wasm.base64_to_z85('SGVsbG8gV29ybGQ=');
      expect(result).toBe('nm=QNzY&b1A+]m^:1');
    });

    it('should handle encode/decode roundtrip', () => {
      const data = new TextEncoder().encode('Hello, World!');
      const encoded = wasm.encode_z85(data);
      const decoded = wasm.decode_z85(encoded);
      const decodedText = new TextDecoder().decode(decoded);
      expect(decodedText).toBe('Hello, World!');
    });
  });

  describe('DataURL conversions', () => {
    it('should convert Z85 DataURL to base64 DataURL', () => {
      const input = 'data:image/png;z85,nm=QNzY&b1A+]m^:1';
      const options = new wasm.ConversionOptions(wasm.DataType.DataURL, wasm.DataType.DataURL);
      const result = wasm.z85_to_base64_with_options(input, options);
      expect(result).toBe('data:image/png;base64,SGVsbG8gV29ybGQ=');
    });

    it('should convert Z85 DataURL to raw base64', () => {
      const input = 'data:image/png;z85,nm=QNzY&b1A+]m^:1';
      const options = new wasm.ConversionOptions(wasm.DataType.DataURL, wasm.DataType.Raw);
      const result = wasm.z85_to_base64_with_options(input, options);
      expect(result).toBe('SGVsbG8gV29ybGQ=');
    });

    it('should convert base64 DataURL to Z85 DataURL', () => {
      const input = 'data:image/jpeg;base64,SGVsbG8gV29ybGQ=';
      const options = new wasm.ConversionOptions(wasm.DataType.DataURL, wasm.DataType.DataURL);
      const result = wasm.base64_to_z85_with_options(input, options);
      expect(result).toBe('data:image/jpeg;z85,nm=QNzY&b1A+]m^:1');
    });

    it('should handle various MIME types', () => {
      const mimeTypes = ['image/png', 'image/jpeg', 'image/webp', 'application/pdf', 'text/plain'];
      
      for (const mime of mimeTypes) {
        const input = `data:${mime};base64,SGVsbG8gV29ybGQ=`;
        const options = new wasm.ConversionOptions(wasm.DataType.DataURL, wasm.DataType.DataURL);
        const result = wasm.base64_to_z85_with_options(input, options);
        expect(result).toMatch(new RegExp(`^data:${mime.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')};z85,`));
      }
    });
  });

  describe('Error handling', () => {
    it('should throw error when converting raw to DataURL', () => {
      const options = new wasm.ConversionOptions(wasm.DataType.Raw, wasm.DataType.DataURL);
      expect(() => {
        wasm.z85_to_base64_with_options('nm=QNzY&b1A+]m^:1', options);
      }).toThrow('Cannot convert raw to data URL: MIME type unknown');
    });

    it('should throw error for invalid Z85 format', () => {
      expect(() => {
        wasm.z85_to_base64('invalid_format');
      }).toThrow("Invalid format: expected 'z85_data:padding'");
    });

    it('should throw error for invalid base64', () => {
      expect(() => {
        wasm.base64_to_z85('not valid base64!');
      }).toThrow('Base64 decode error');
    });
  });

  describe('Edge cases', () => {
    it('should handle empty input', () => {
      const data = new Uint8Array(0);
      const encoded = wasm.encode_z85(data);
      const decoded = wasm.decode_z85(encoded);
      expect(decoded.length).toBe(0);
    });

    it('should handle large data', () => {
      const largeData = new Uint8Array(10000).fill(65); // 10KB of 'A'
      const encoded = wasm.encode_z85(largeData);
      const decoded = wasm.decode_z85(encoded);
      expect(decoded).toEqual(largeData);
    });

    it('should handle various padding scenarios', () => {
      const testCases = [
        { input: 'A', expectedPaddingNeeded: 3 },
        { input: 'AB', expectedPaddingNeeded: 2 },
        { input: 'ABC', expectedPaddingNeeded: 1 },
        { input: 'ABCD', expectedPaddingNeeded: 0 },
        { input: 'ABCDE', expectedPaddingNeeded: 3 },
      ];

      for (const testCase of testCases) {
        const data = new TextEncoder().encode(testCase.input);
        const encoded = wasm.encode_z85(data);
        const parts = encoded.split(':');
        const padding = parseInt(parts[parts.length - 1]);
        expect(padding).toBe(testCase.expectedPaddingNeeded);
        
        // Verify roundtrip
        const decoded = wasm.decode_z85(encoded);
        const decodedText = new TextDecoder().decode(decoded);
        expect(decodedText).toBe(testCase.input);
      }
    });
  });
});