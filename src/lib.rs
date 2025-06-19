// Implement extract_tx_version function below
#[allow(unused_variables)]
pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
    if raw_tx_hex.len() < 8 {
        return Err("Transaction data too short".to_string());
    }

    // Try to decode the first 8 characters (4 bytes)
    let version_hex = &raw_tx_hex[0..8];
    let version_bytes = hex::decode(version_hex)
        .map_err(|_| "Hex decode error".to_string())?;

    if version_bytes.len() != 4 {
        return Err("Decoded version length is not 4 bytes".to_string());
    }

    // Convert from little-endian bytes to a u32
    let version = u32::from_le_bytes([
        version_bytes[0],
        version_bytes[1],
        version_bytes[2],
        version_bytes[3],
    ]);

    Ok(version)
}