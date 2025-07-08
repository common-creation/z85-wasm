use wasm_bindgen::prelude::*;
use base64::{Engine, engine::general_purpose};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

/// Data type for conversion
#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub enum DataType {
    /// Raw format (z85data:padding or base64data)
    Raw,
    /// Data URL format (data:mime/type;encoding,data)
    DataURL,
}

/// Conversion options
#[wasm_bindgen]
pub struct ConversionOptions {
    input: DataType,
    output: DataType,
}

#[wasm_bindgen]
impl ConversionOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(input: DataType, output: DataType) -> ConversionOptions {
        ConversionOptions { input, output }
    }

    #[wasm_bindgen(getter)]
    pub fn input(&self) -> DataType {
        self.input
    }

    #[wasm_bindgen(setter)]
    pub fn set_input(&mut self, input: DataType) {
        self.input = input;
    }

    #[wasm_bindgen(getter)]
    pub fn output(&self) -> DataType {
        self.output
    }

    #[wasm_bindgen(setter)]
    pub fn set_output(&mut self, output: DataType) {
        self.output = output;
    }
}

// Internal pure Rust function for Z85 to base64 conversion
fn z85_to_base64_internal(z85_data_with_padding: &str) -> Result<String, String> {
    // Parse Z85 data and padding info - split by the LAST colon
    let last_colon_pos = z85_data_with_padding.rfind(':');
    if last_colon_pos.is_none() {
        return Err("Invalid format: expected 'z85_data:padding'".to_string());
    }
    
    let colon_pos = last_colon_pos.unwrap();
    let z85_data = &z85_data_with_padding[..colon_pos];
    let padding_str = &z85_data_with_padding[colon_pos + 1..];
    
    let padding: usize = padding_str.parse()
        .map_err(|_| "Invalid padding number".to_string())?;
    
    // Decode Z85 data
    let decoded_data = z85::decode(z85_data)
        .map_err(|e| format!("Z85 decode error: {}", e))?;
    
    // Remove padding
    let original_length = decoded_data.len() - padding;
    let trimmed_data = &decoded_data[..original_length];
    
    // Encode to base64
    let base64_data = general_purpose::STANDARD.encode(trimmed_data);
    
    Ok(base64_data)
}

/// Convert Z85 encoded data with padding info to base64
#[wasm_bindgen]
pub fn z85_to_base64(z85_data_with_padding: &str) -> Result<String, JsValue> {
    z85_to_base64_internal(z85_data_with_padding)
        .map_err(|e| JsValue::from_str(&e))
}

// Internal pure Rust function for Z85 to base64 conversion with options
fn z85_to_base64_with_options_internal(data: &str, input_type: DataType, output_type: DataType) -> Result<String, String> {
    match (input_type, output_type) {
        (DataType::Raw, DataType::Raw) => {
            // Use existing logic
            z85_to_base64_internal(data)
        }
        (DataType::DataURL, DataType::DataURL) => {
            // Parse data URL
            if !data.starts_with("data:") {
                return Err("Invalid data URL format".to_string());
            }
            
            // Find ;z85,
            if let Some(z85_pos) = data.find(";z85,") {
                let mime_type = &data[5..z85_pos];
                let z85_data = &data[z85_pos + 5..];
                
                // Convert Z85 to base64
                let base64_data = z85_to_base64_internal(z85_data)?;
                
                // Reconstruct data URL with base64
                Ok(format!("data:{};base64,{}", mime_type, base64_data))
            } else {
                Err("Data URL does not contain ;z85, marker".to_string())
            }
        }
        (DataType::DataURL, DataType::Raw) => {
            // Extract Z85 data from data URL and convert to raw base64
            if !data.starts_with("data:") {
                return Err("Invalid data URL format".to_string());
            }
            
            if let Some(z85_pos) = data.find(";z85,") {
                let z85_data = &data[z85_pos + 5..];
                z85_to_base64_internal(z85_data)
            } else {
                Err("Data URL does not contain ;z85, marker".to_string())
            }
        }
        (DataType::Raw, DataType::DataURL) => {
            Err("Cannot convert raw to data URL: MIME type unknown".to_string())
        }
    }
}

/// Convert Z85 encoded data to base64 with options
#[wasm_bindgen]
pub fn z85_to_base64_with_options(data: &str, options: Option<ConversionOptions>) -> Result<String, JsValue> {
    let opts = options.unwrap_or(ConversionOptions::new(DataType::Raw, DataType::Raw));
    z85_to_base64_with_options_internal(data, opts.input, opts.output)
        .map_err(|e| JsValue::from_str(&e))
}

// Internal pure Rust function for base64 to Z85 conversion
fn base64_to_z85_internal(base64_data: &str) -> Result<String, String> {
    // Decode base64 data
    let decoded_data = general_purpose::STANDARD.decode(base64_data)
        .map_err(|e| format!("Base64 decode error: {}", e))?;
    
    // Calculate padding needed (Z85 requires length divisible by 4)
    let padding_needed = (4 - (decoded_data.len() % 4)) % 4;
    let mut padded_data = decoded_data.clone();
    
    // Add padding bytes
    for _ in 0..padding_needed {
        padded_data.push(0);
    }
    
    // Encode to Z85
    let z85_data = z85::encode(&padded_data);
    
    // Return with padding info
    Ok(format!("{}:{}", z85_data, padding_needed))
}

/// Convert base64 data to Z85 with padding info
#[wasm_bindgen]
pub fn base64_to_z85(base64_data: &str) -> Result<String, JsValue> {
    base64_to_z85_internal(base64_data)
        .map_err(|e| JsValue::from_str(&e))
}

// Internal pure Rust function for base64 to Z85 conversion with options
fn base64_to_z85_with_options_internal(data: &str, input_type: DataType, output_type: DataType) -> Result<String, String> {
    match (input_type, output_type) {
        (DataType::Raw, DataType::Raw) => {
            // Use existing logic
            base64_to_z85_internal(data)
        }
        (DataType::DataURL, DataType::DataURL) => {
            // Parse data URL
            if !data.starts_with("data:") {
                return Err("Invalid data URL format".to_string());
            }
            
            // Find ;base64,
            if let Some(base64_pos) = data.find(";base64,") {
                let mime_type = &data[5..base64_pos];
                let base64_data = &data[base64_pos + 8..];
                
                // Convert base64 to Z85
                let z85_data = base64_to_z85_internal(base64_data)?;
                
                // Reconstruct data URL with z85
                Ok(format!("data:{};z85,{}", mime_type, z85_data))
            } else {
                Err("Data URL does not contain ;base64, marker".to_string())
            }
        }
        (DataType::DataURL, DataType::Raw) => {
            // Extract base64 data from data URL and convert to raw Z85
            if !data.starts_with("data:") {
                return Err("Invalid data URL format".to_string());
            }
            
            if let Some(base64_pos) = data.find(";base64,") {
                let base64_data = &data[base64_pos + 8..];
                base64_to_z85_internal(base64_data)
            } else {
                Err("Data URL does not contain ;base64, marker".to_string())
            }
        }
        (DataType::Raw, DataType::DataURL) => {
            Err("Cannot convert raw to data URL: MIME type unknown".to_string())
        }
    }
}

/// Convert base64 data to Z85 with options
#[wasm_bindgen]
pub fn base64_to_z85_with_options(data: &str, options: Option<ConversionOptions>) -> Result<String, JsValue> {
    let opts = options.unwrap_or(ConversionOptions::new(DataType::Raw, DataType::Raw));
    base64_to_z85_with_options_internal(data, opts.input, opts.output)
        .map_err(|e| JsValue::from_str(&e))
}

// Internal pure Rust function for encoding bytes to Z85
fn encode_z85_internal(data: &[u8]) -> String {
    // Calculate padding needed (Z85 requires length divisible by 4)
    let padding_needed = (4 - (data.len() % 4)) % 4;
    let mut padded_data = data.to_vec();
    
    // Add padding bytes
    for _ in 0..padding_needed {
        padded_data.push(0);
    }
    
    // Encode to Z85
    let z85_data = z85::encode(&padded_data);
    
    // Return with padding info
    format!("{}:{}", z85_data, padding_needed)
}

/// Encode raw bytes to Z85 with padding info
#[wasm_bindgen]
pub fn encode_z85(data: &[u8]) -> Result<String, JsValue> {
    Ok(encode_z85_internal(data))
}

// Internal pure Rust function for decoding Z85 to bytes
fn decode_z85_internal(z85_data_with_padding: &str) -> Result<Vec<u8>, String> {
    // Parse Z85 data and padding info - split by the LAST colon
    let last_colon_pos = z85_data_with_padding.rfind(':');
    if last_colon_pos.is_none() {
        return Err("Invalid format: expected 'z85_data:padding'".to_string());
    }
    
    let colon_pos = last_colon_pos.unwrap();
    let z85_data = &z85_data_with_padding[..colon_pos];
    let padding_str = &z85_data_with_padding[colon_pos + 1..];
    
    let padding: usize = padding_str.parse()
        .map_err(|_| "Invalid padding number".to_string())?;
    
    // Decode Z85 data
    let mut decoded_data = z85::decode(z85_data)
        .map_err(|e| format!("Z85 decode error: {}", e))?;
    
    // Remove padding
    if padding > 0 {
        decoded_data.truncate(decoded_data.len() - padding);
    }
    
    Ok(decoded_data)
}

/// Decode Z85 data with padding info to raw bytes
#[wasm_bindgen]
pub fn decode_z85(z85_data_with_padding: &str) -> Result<Vec<u8>, JsValue> {
    decode_z85_internal(z85_data_with_padding)
        .map_err(|e| JsValue::from_str(&e))
}

// Internal pure Rust function for calculating encoding efficiency
fn get_encoding_efficiency_internal(original_size: usize) -> serde_json::Value {
    let base64_size = (original_size + 2) / 3 * 4; // Base64: 3 bytes -> 4 chars
    let z85_size = (original_size + 3) / 4 * 5;     // Z85: 4 bytes -> 5 chars
    
    let efficiency_ratio = z85_size as f64 / base64_size as f64;
    
    serde_json::json!({
        "original_size": original_size,
        "base64_size": base64_size,
        "z85_size": z85_size,
        "efficiency_ratio": efficiency_ratio,
        "bandwidth_saving": (1.0 - efficiency_ratio) * 100.0
    })
}

/// Get encoding efficiency comparison
#[wasm_bindgen]
pub fn get_encoding_efficiency(original_size: usize) -> JsValue {
    serde_wasm_bindgen::to_value(&get_encoding_efficiency_internal(original_size)).unwrap()
}

/// Initialize the WASM module
#[wasm_bindgen(start)]
pub fn init_wasm() {
    console_log!("Z85 encoder/decoder WASM module initialized");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Standard Rust tests
    #[test]
    fn test_z85_to_base64_basic() {
        // "Hello World" encoded in Z85 with padding
        let z85_with_padding = "nm=QNzY&b1A+]m^:1";
        let expected = "SGVsbG8gV29ybGQ=";
        let result = z85_to_base64_internal(z85_with_padding).unwrap();
        assert_eq!(result, expected);
    }
    
    #[test]
    fn test_base64_to_z85_basic() {
        let base64 = "SGVsbG8gV29ybGQ=";
        let result = base64_to_z85_internal(base64).unwrap();
        // Verify it's in the format "z85data:padding"
        assert!(result.contains(':'));
        // Verify roundtrip
        let back = z85_to_base64_internal(&result).unwrap();
        assert_eq!(back, base64);
    }
    
    #[test]
    fn test_encode_decode_z85_roundtrip() {
        let data = b"Hello, World!";
        let encoded = encode_z85_internal(data);
        assert!(encoded.contains(':'));
        let decoded = decode_z85_internal(&encoded).unwrap();
        assert_eq!(data, decoded.as_slice());
    }
    
    #[test]
    fn test_conversion_options() {
        // Test constructor
        let opts = ConversionOptions::new(DataType::DataURL, DataType::Raw);
        
        // Test getters
        assert!(matches!(opts.input(), DataType::DataURL));
        assert!(matches!(opts.output(), DataType::Raw));
        
        // Test setters
        let mut opts = ConversionOptions::new(DataType::Raw, DataType::Raw);
        opts.set_input(DataType::DataURL);
        opts.set_output(DataType::DataURL);
        assert!(matches!(opts.input(), DataType::DataURL));
        assert!(matches!(opts.output(), DataType::DataURL));
    }
    
    #[test]
    fn test_z85_to_base64_with_options_all_branches() {
        let base64 = "SGVsbG8gV29ybGQ=";
        let z85_data = base64_to_z85_internal(base64).unwrap();
        
        // Test Raw -> Raw (default behavior)
        let result = z85_to_base64_with_options_internal(&z85_data, DataType::Raw, DataType::Raw).unwrap();
        assert_eq!(result, base64);
        
        // Test DataURL -> DataURL
        let input = format!("data:image/png;z85,{}", z85_data);
        let result = z85_to_base64_with_options_internal(&input, DataType::DataURL, DataType::DataURL).unwrap();
        assert_eq!(result, "data:image/png;base64,SGVsbG8gV29ybGQ=");
        
        // Test DataURL -> Raw
        let result = z85_to_base64_with_options_internal(&input, DataType::DataURL, DataType::Raw).unwrap();
        assert_eq!(result, base64);
        
        // Test Raw -> DataURL (should error)
        let result = z85_to_base64_with_options_internal(&z85_data, DataType::Raw, DataType::DataURL);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_base64_to_z85_with_options_all_branches() {
        let base64 = "SGVsbG8gV29ybGQ=";
        
        // Test Raw -> Raw (default behavior)
        let result = base64_to_z85_with_options_internal(base64, DataType::Raw, DataType::Raw).unwrap();
        assert!(result.contains(':'));
        let back = z85_to_base64_internal(&result).unwrap();
        assert_eq!(back, base64);
        
        // Test DataURL -> DataURL
        let input = format!("data:image/jpeg;base64,{}", base64);
        let result = base64_to_z85_with_options_internal(&input, DataType::DataURL, DataType::DataURL).unwrap();
        assert!(result.starts_with("data:image/jpeg;z85,"));
        assert!(result.contains(':'));
        
        // Test DataURL -> Raw
        let result = base64_to_z85_with_options_internal(&input, DataType::DataURL, DataType::Raw).unwrap();
        assert!(!result.starts_with("data:"));
        assert!(result.contains(':'));
        
        // Test Raw -> DataURL (should error)
        let result = base64_to_z85_with_options_internal(base64, DataType::Raw, DataType::DataURL);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_dataurl_error_cases() {
        // Invalid data URL format
        let result = z85_to_base64_with_options_internal("not_a_dataurl", DataType::DataURL, DataType::DataURL);
        assert!(result.is_err());
        
        // Missing ;z85, marker
        let result = z85_to_base64_with_options_internal("data:image/png;base64,data", DataType::DataURL, DataType::DataURL);
        assert!(result.is_err());
        
        // Invalid data URL for base64
        let result = base64_to_z85_with_options_internal("not_a_dataurl", DataType::DataURL, DataType::DataURL);
        assert!(result.is_err());
        
        // Missing ;base64, marker
        let result = base64_to_z85_with_options_internal("data:image/png;z85,data", DataType::DataURL, DataType::DataURL);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_multiple_colons_in_z85() {
        // Test data with multiple colons to verify rfind logic
        // Create valid Z85 data with colons in it
        let z85_data = "nm=QNzY&b1A+]m^";  // Valid Z85 data
        let z85_with_colons = format!("data:with:{}:1", z85_data);
        let result = z85_to_base64_internal(&z85_with_colons);
        // Should split at the LAST colon and decode successfully
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_get_encoding_efficiency_internal() {
        // Test the internal function
        let stats = get_encoding_efficiency_internal(1000);
        assert_eq!(stats["original_size"], 1000);
        assert_eq!(stats["base64_size"], 1336);
        assert_eq!(stats["z85_size"], 1250);
        
        let efficiency_ratio = stats["efficiency_ratio"].as_f64().unwrap();
        assert!(efficiency_ratio > 0.93 && efficiency_ratio < 0.94);
        
        let bandwidth_saving = stats["bandwidth_saving"].as_f64().unwrap();
        assert!(bandwidth_saving > 6.0 && bandwidth_saving < 7.0);
    }
    
    #[test]
    fn test_large_data() {
        // Test with larger data
        let large_data = vec![b'A'; 1000];
        let encoded = encode_z85_internal(&large_data);
        let decoded = decode_z85_internal(&encoded).unwrap();
        assert_eq!(large_data, decoded);
    }
    
    #[test]
    fn test_padding_edge_cases() {
        // Test data that requires different padding amounts
        let test_cases: Vec<(&[u8], usize)> = vec![
            (b"H", 3),     // 1 byte -> needs 3 padding
            (b"He", 2),    // 2 bytes -> needs 2 padding
            (b"Hel", 1),   // 3 bytes -> needs 1 padding
            (b"Hell", 0),  // 4 bytes -> needs 0 padding
            (b"Hello", 3), // 5 bytes -> needs 3 padding
        ];
        
        for (data, expected_padding) in test_cases {
            let encoded = encode_z85_internal(data);
            let parts: Vec<&str> = encoded.rsplitn(2, ':').collect();
            let padding: usize = parts[0].parse().unwrap();
            assert_eq!(padding, expected_padding, "Padding mismatch for {:?}", data);
            
            // Verify roundtrip
            let decoded = decode_z85_internal(&encoded).unwrap();
            assert_eq!(data, decoded.as_slice());
        }
    }
    
    #[test]
    fn test_empty_input() {
        let encoded = encode_z85_internal(b"");
        let decoded = decode_z85_internal(&encoded).unwrap();
        assert_eq!(decoded.as_slice(), b"");
    }
    
    #[test]
    fn test_efficiency_calculation() {
        // Test the efficiency calculation logic manually
        let original_size = 100000;
        let base64_size = (original_size + 2) / 3 * 4;
        let z85_size = (original_size + 3) / 4 * 5;
        
        assert_eq!(base64_size, 133336);
        assert_eq!(z85_size, 125000);
        
        let efficiency_ratio = z85_size as f64 / base64_size as f64;
        assert!(efficiency_ratio > 0.93 && efficiency_ratio < 0.94);
        
        let bandwidth_saving = (1.0 - efficiency_ratio) * 100.0;
        assert!(bandwidth_saving > 6.0 && bandwidth_saving < 7.0);
    }
    
    #[test]
    fn test_z85_to_base64_errors() {
        // Test missing colon
        let result = z85_to_base64_internal("no_colon_here");
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Invalid format: expected 'z85_data:padding'");
        
        // Test invalid padding number
        let result = z85_to_base64_internal("data:not_a_number");
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Invalid padding number");
        
        // Test invalid Z85 data
        let result = z85_to_base64_internal("invalid_z85!:0");
        assert!(result.is_err());
        assert!(result.err().unwrap().contains("Z85 decode error"));
    }
    
    #[test]
    fn test_base64_to_z85_errors() {
        // Test invalid base64
        let result = base64_to_z85_internal("not valid base64!");
        assert!(result.is_err());
        assert!(result.err().unwrap().contains("Base64 decode error"));
    }
    
    #[test]
    fn test_decode_z85_errors() {
        // Test missing colon
        let result = decode_z85_internal("no_colon_here");
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Invalid format: expected 'z85_data:padding'");
        
        // Test invalid padding number
        let result = decode_z85_internal("data:not_a_number");
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Invalid padding number");
        
        // Test invalid Z85 data
        let result = decode_z85_internal("invalid_z85!:0");
        assert!(result.is_err());
        assert!(result.err().unwrap().contains("Z85 decode error"));
    }
    
    #[test]
    fn test_dataurl_edge_cases() {
        // Test DataURL -> Raw with invalid format
        let result = z85_to_base64_with_options_internal("not_data_url", DataType::DataURL, DataType::Raw);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Invalid data URL format");
        
        // Test DataURL -> Raw with missing marker
        let result = z85_to_base64_with_options_internal("data:image/png;base64,data", DataType::DataURL, DataType::Raw);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Data URL does not contain ;z85, marker");
        
        // Test base64 DataURL -> Raw with invalid format
        let result = base64_to_z85_with_options_internal("not_data_url", DataType::DataURL, DataType::Raw);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Invalid data URL format");
        
        // Test base64 DataURL -> Raw with missing marker
        let result = base64_to_z85_with_options_internal("data:image/png;z85,data", DataType::DataURL, DataType::Raw);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Data URL does not contain ;base64, marker");
    }
    
    #[test]
    fn test_conversion_options_setters() {
        let mut opts = ConversionOptions::new(DataType::Raw, DataType::Raw);
        
        // Test initial values
        assert!(matches!(opts.input(), DataType::Raw));
        assert!(matches!(opts.output(), DataType::Raw));
        
        // Test setters
        opts.set_input(DataType::DataURL);
        assert!(matches!(opts.input(), DataType::DataURL));
        
        opts.set_output(DataType::DataURL);
        assert!(matches!(opts.output(), DataType::DataURL));
    }
    
    #[test]
    fn test_more_mime_types() {
        let mime_types = vec![
            "application/json",
            "text/html",
            "video/mp4",
            "audio/mpeg",
            "application/octet-stream",
        ];
        
        for mime_type in mime_types {
            // Test Z85 DataURL -> Base64 DataURL
            let z85_data = base64_to_z85_internal("SGVsbG8gV29ybGQ=").unwrap();
            let input = format!("data:{};z85,{}", mime_type, z85_data);
            let result = z85_to_base64_with_options_internal(&input, DataType::DataURL, DataType::DataURL).unwrap();
            assert_eq!(result, format!("data:{};base64,SGVsbG8gV29ybGQ=", mime_type));
            
            // Test Base64 DataURL -> Z85 DataURL
            let input = format!("data:{};base64,SGVsbG8gV29ybGQ=", mime_type);
            let result = base64_to_z85_with_options_internal(&input, DataType::DataURL, DataType::DataURL).unwrap();
            assert!(result.starts_with(&format!("data:{};z85,", mime_type)));
        }
    }
    
    #[test]
    fn test_various_data_sizes() {
        // Test various data sizes to ensure proper padding handling
        let data_100 = vec![b'X'; 100];
        let data_1000 = vec![b'Y'; 1000];
        let data_10000 = vec![b'Z'; 10000];
        
        let test_data = vec![
            b"" as &[u8],           // 0 bytes
            b"A",                   // 1 byte
            b"AB",                  // 2 bytes
            b"ABC",                 // 3 bytes
            b"ABCD",                // 4 bytes
            b"ABCDE",               // 5 bytes
            b"ABCDEF",              // 6 bytes
            b"ABCDEFG",             // 7 bytes
            b"ABCDEFGH",            // 8 bytes
            &data_100,              // 100 bytes
            &data_1000,             // 1000 bytes
            &data_10000,            // 10000 bytes
        ];
        
        for data in test_data {
            // Test encode/decode roundtrip
            let encoded = encode_z85_internal(data);
            let decoded = decode_z85_internal(&encoded).unwrap();
            assert_eq!(data, decoded.as_slice(), "Failed for data size: {}", data.len());
            
            // Test base64 conversion roundtrip
            let base64 = general_purpose::STANDARD.encode(data);
            let z85 = base64_to_z85_internal(&base64).unwrap();
            let back_to_base64 = z85_to_base64_internal(&z85).unwrap();
            assert_eq!(base64, back_to_base64, "Failed base64 roundtrip for data size: {}", data.len());
        }
    }
    
    // WASM-specific tests (kept for wasm-pack test)
    #[cfg(target_arch = "wasm32")]
    mod wasm_tests {
        use super::*;
        use wasm_bindgen_test::*;
        
        #[wasm_bindgen_test]
        fn wasm_test_basic_roundtrip() {
            let original = "SGVsbG8gV29ybGQ=";
            let z85_result = base64_to_z85(original).unwrap();
            let back_to_base64 = z85_to_base64(&z85_result).unwrap();
            assert_eq!(original, back_to_base64);
        }
        
        #[wasm_bindgen_test]
        fn wasm_test_encode_decode() {
            let data = b"Hello, World!";
            let encoded = encode_z85(data).unwrap();
            let decoded = decode_z85(&encoded).unwrap();
            assert_eq!(data, decoded.as_slice());
        }
        
        #[wasm_bindgen_test]
        fn wasm_test_with_options() {
            let opts = ConversionOptions::new(DataType::DataURL, DataType::DataURL);
            let input = "data:image/png;z85,nm=QNzY&b1A+]m^:1";
            let result = z85_to_base64_with_options(input, Some(opts)).unwrap();
            assert_eq!(result, "data:image/png;base64,SGVsbG8gV29ybGQ=");
        }
        
        #[wasm_bindgen_test]
        fn wasm_test_efficiency() {
            let result = get_encoding_efficiency(1000);
            // Just verify it doesn't panic
            assert!(result.is_object());
        }
    }
}