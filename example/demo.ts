import init, {
  z85_to_base64,
  base64_to_z85,
  encode_z85,
  decode_z85,
  z85_to_base64_with_options,
  base64_to_z85_with_options,
  get_encoding_efficiency,
  ConversionOptions,
  DataType
} from '../node';

async function main() {
  // Initialize the WASM module
  console.log('Initializing WASM module...');
  await init();
  console.log('WASM module initialized!\n');

  // Example 1: Basic Base64 ↔ Z85 conversion
  console.log('=== Example 1: Basic Base64 ↔ Z85 conversion ===');
  const originalBase64 = "SGVsbG8gV29ybGQ="; // "Hello World" in base64
  console.log(`Original Base64: ${originalBase64}`);
  
  const z85Data = base64_to_z85(originalBase64);
  console.log(`Converted to Z85: ${z85Data}`);
  
  const backToBase64 = z85_to_base64(z85Data);
  console.log(`Converted back to Base64: ${backToBase64}`);
  console.log(`Match: ${originalBase64 === backToBase64}\n`);

  // Example 2: Encode/Decode raw data
  console.log('=== Example 2: Encode/Decode raw data ===');
  const textData = "Hello, Z85 encoding!";
  const rawData = new TextEncoder().encode(textData);
  console.log(`Original text: ${textData}`);
  console.log(`Raw bytes: [${Array.from(rawData).join(', ')}]`);
  
  const encodedZ85 = encode_z85(rawData);
  console.log(`Encoded to Z85: ${encodedZ85}`);
  
  const decodedData = decode_z85(encodedZ85);
  const decodedText = new TextDecoder().decode(decodedData);
  console.log(`Decoded text: ${decodedText}`);
  console.log(`Match: ${textData === decodedText}\n`);

  // Example 3: Data URL conversion
  console.log('=== Example 3: Data URL conversion ===');
  // Create a small image data (1x1 red pixel PNG)
  const redPixelBase64 = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8/5+hHgAHggJ/PchI7wAAAABJRU5ErkJggg==";
  const imageDataUrl = `data:image/png;base64,${redPixelBase64}`;
  console.log(`Original Data URL: ${imageDataUrl.substring(0, 50)}...`);
  
  // Convert Data URL from Base64 to Z85
  const z85DataUrl = base64_to_z85_with_options(
    imageDataUrl,
    new ConversionOptions(DataType.DataURL, DataType.DataURL)
  );
  console.log(`Z85 Data URL: ${z85DataUrl.substring(0, 50)}...`);
  
  // Convert back to Base64 Data URL
  const backToBase64DataUrl = z85_to_base64_with_options(
    z85DataUrl,
    new ConversionOptions(DataType.DataURL, DataType.DataURL)
  );
  console.log(`Back to Base64 Data URL: ${backToBase64DataUrl.substring(0, 50)}...`);
  console.log(`Match: ${imageDataUrl === backToBase64DataUrl}\n`);

  // Example 4: Extract raw data from Data URL
  console.log('=== Example 4: Extract raw data from Data URL ===');
  const rawZ85FromDataUrl = base64_to_z85_with_options(
    imageDataUrl,
    new ConversionOptions(DataType.DataURL, DataType.Raw)
  );
  console.log(`Raw Z85 data extracted: ${rawZ85FromDataUrl.substring(0, 50)}...`);
  
  const rawBase64FromZ85 = z85_to_base64_with_options(
    z85DataUrl,
    new ConversionOptions(DataType.DataURL, DataType.Raw)
  );
  console.log(`Raw Base64 data extracted: ${rawBase64FromZ85.substring(0, 50)}...`);
  console.log(`Match with original: ${redPixelBase64 === rawBase64FromZ85}\n`);

  // Example 5: Encoding efficiency comparison
  console.log('=== Example 5: Encoding efficiency comparison ===');
  const dataSizes = [100, 1000, 10000, 100000, 1000000];
  
  for (const size of dataSizes) {
    const stats = get_encoding_efficiency(size);
    // The result is a Map in Node.js
    const statsMap = stats as Map<string, number>;
    console.log(`Data size: ${size.toLocaleString()} bytes`);
    console.log(`  Base64 size: ${statsMap.get('base64_size')?.toLocaleString()} bytes`);
    console.log(`  Z85 size: ${statsMap.get('z85_size')?.toLocaleString()} bytes`);
    console.log(`  Efficiency ratio: ${statsMap.get('efficiency_ratio')?.toFixed(4)}`);
    console.log(`  Bandwidth saving: ${statsMap.get('bandwidth_saving')?.toFixed(2)}%`);
  }
  console.log('\n');

  // Example 6: Different padding scenarios
  console.log('=== Example 6: Different padding scenarios ===');
  const testStrings = ["A", "AB", "ABC", "ABCD", "ABCDE"];
  
  for (const str of testStrings) {
    const bytes = new TextEncoder().encode(str);
    const encoded = encode_z85(bytes);
    const [z85Part, paddingPart] = encoded.split(':');
    console.log(`String: "${str}" (${bytes.length} bytes)`);
    console.log(`  Z85: ${z85Part}`);
    console.log(`  Padding: ${paddingPart}`);
    
    const decoded = decode_z85(encoded);
    const decodedStr = new TextDecoder().decode(decoded);
    console.log(`  Decoded: "${decodedStr}" (match: ${str === decodedStr})`);
  }
  console.log('\n');

  // Example 7: Error handling
  console.log('=== Example 7: Error handling ===');
  
  try {
    // Invalid Z85 format (missing padding info)
    z85_to_base64("invalid_z85_without_padding");
  } catch (error) {
    console.log(`Expected error for invalid format: ${error}`);
  }
  
  try {
    // Invalid base64
    base64_to_z85("not valid base64!");
  } catch (error) {
    console.log(`Expected error for invalid base64: ${error}`);
  }
  
  try {
    // Invalid Data URL format
    z85_to_base64_with_options(
      "not_a_data_url",
      new ConversionOptions(DataType.DataURL, DataType.DataURL)
    );
  } catch (error) {
    console.log(`Expected error for invalid Data URL: ${error}`);
  }

  console.log('\nDemo completed!');
}

// Run the demo
main().catch(console.error);