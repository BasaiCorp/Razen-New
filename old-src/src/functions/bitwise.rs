use crate::value::Value;

/// Perform bitwise AND operation
/// Example: and(5, 3) => 1
pub fn and(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("Bitwise.and requires exactly 2 arguments: a, b".to_string());
    }
    
    let a = args[0].as_int()? as i64;
    let b = args[1].as_int()? as i64;
    
    Ok(Value::Int(a & b))
}

/// Perform bitwise OR operation
/// Example: or(5, 3) => 7
pub fn or(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("Bitwise.or requires exactly 2 arguments: a, b".to_string());
    }
    
    let a = args[0].as_int()? as i64;
    let b = args[1].as_int()? as i64;
    
    Ok(Value::Int(a | b))
}

/// Perform bitwise XOR operation
/// Example: xor(5, 3) => 6
pub fn xor(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("Bitwise.xor requires exactly 2 arguments: a, b".to_string());
    }
    
    let a = args[0].as_int()? as i64;
    let b = args[1].as_int()? as i64;
    
    Ok(Value::Int(a ^ b))
}

/// Perform bitwise NOT operation
/// Example: not(5, 8) => 250 (for 8-bit complement)
pub fn not(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("Bitwise.not requires exactly 2 arguments: value, bits".to_string());
    }
    
    let value = args[0].as_int()? as i64;
    let bits = args[1].as_int()? as u32;
    
    if bits > 64 {
        return Err("Bit width must be between 1 and 64".to_string());
    }
    
    // Create a mask with the specified Int of bits
    let mask = if bits == 64 {
        !0 // All bits set for 64-bit
    } else {
        (1 << bits) - 1
    };
    
    // Perform bitwise NOT and apply the mask
    let result = (!value) & mask;
    
    Ok(Value::Int(result))
}

/// Perform left shift operation
/// Example: left_shift(5, 2) => 20
pub fn left_shift(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("Bitwise.left_shift requires exactly 2 arguments: value, shift".to_string());
    }
    
    let value = args[0].as_int()? as i64;
    let shift = args[1].as_int()? as u32;
    
    if shift > 63 {
        return Err("Shift amount must be between 0 and 63".to_string());
    }
    
    Ok(Value::Int(value << shift))
}

/// Perform right shift operation
/// Example: right_shift(5, 1) => 2
pub fn right_shift(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("Bitwise.right_shift requires exactly 2 arguments: value, shift".to_string());
    }
    
    let value = args[0].as_int()? as i64;
    let shift = args[1].as_int()? as u32;
    
    if shift > 63 {
        return Err("Shift amount must be between 0 and 63".to_string());
    }
    
    Ok(Value::Int(value >> shift))
}

/// Perform unsigned right shift operation
/// Example: unsigned_right_shift(5, 1) => 2
pub fn unsigned_right_shift(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("Bitwise.unsigned_right_shift requires exactly 2 arguments: value, shift".to_string());
    }
    
    let value = args[0].as_int()? as u64;
    let shift = args[1].as_int()? as u32;
    
    if shift > 63 {
        return Err("Shift amount must be between 0 and 63".to_string());
    }
    
    Ok(Value::Int((value >> shift) as i64))
}

/// Get a specific bit from a value
/// Example: get_bit(5, 0) => 1 (5 in binary is 101, bit 0 is 1)
pub fn get_bit(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("Bitwise.get_bit requires exactly 2 arguments: value, bit_position".to_string());
    }
    
    let value = args[0].as_int()? as i64;
    let bit_position = args[1].as_int()? as u32;
    
    if bit_position > 63 {
        return Err("Bit position must be between 0 and 63".to_string());
    }
    
    let bit = (value >> bit_position) & 1;
    
    Ok(Value::Int(bit))
}

/// Set a specific bit in a value
/// Example: set_bit(5, 1, 1) => 7 (5 in binary is 101, setting bit 1 gives 111 which is 7)
pub fn set_bit(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 3 {
        return Err("Bitwise.set_bit requires exactly 3 arguments: value, bit_position, bit_value".to_string());
    }
    
    let value = args[0].as_int()? as i64;
    let bit_position = args[1].as_int()? as u32;
    let bit_value = args[2].as_int()? as i64;
    
    if bit_position > 63 {
        return Err("Bit position must be between 0 and 63".to_string());
    }
    
    if bit_value != 0 && bit_value != 1 {
        return Err("Bit value must be 0 or 1".to_string());
    }
    
    let result = if bit_value == 1 {
        value | (1 << bit_position) // Set bit
    } else {
        value & !(1 << bit_position) // Clear bit
    };
    
    Ok(Value::Int(result))
}

/// Count the Int of set bits (1s) in a value
/// Example: count_bits(5) => 2 (5 in binary is 101, which has two 1s)
pub fn count_bits(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Bitwise.count_bits requires exactly 1 argument: value".to_string());
    }
    
    let value = args[0].as_int()? as i64;
    
    // Count the Int of 1 bits
    let count = value.count_ones();
    
    Ok(Value::Int(count as i64))
}

/// Convert a value to its binary string representation
/// Example: to_binary(5) => "101"
pub fn to_binary(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Bitwise.to_binary requires exactly 1 argument: value".to_string());
    }
    
    let value = args[0].as_int()? as i64;
    
    // Convert to binary string without leading zeros
    let binary = format!("{:b}", value);
    
    Ok(Value::String(binary))
}

/// Convert a value to its hexadecimal string representation
/// Example: to_hex(255) => "ff"
pub fn to_hex(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Bitwise.to_hex requires exactly 1 argument: value".to_string());
    }
    
    let value = args[0].as_int()? as i64;
    
    // Convert to hexadecimal string without leading zeros
    let hex = format!("{:x}", value);
    
    Ok(Value::String(hex))
}

/// Parse a binary string to its numeric value
/// Example: from_binary("101") => 5
pub fn from_binary(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Bitwise.from_binary requires exactly 1 argument: binary_string".to_string());
    }
    
    let binary = args[0].as_string()?;
    
    // Parse binary string
    match i64::from_str_radix(&binary, 2) {
        Ok(value) => Ok(Value::Int(value)),
        Err(e) => Err(format!("Failed to parse binary string: {}", e)),
    }
}

/// Parse a hexadecimal string to its numeric value
/// Example: from_hex("ff") => 255
pub fn from_hex(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Bitwise.from_hex requires exactly 1 argument: hex_string".to_string());
    }
    
    let hex = args[0].as_string()?;
    
    // Parse hexadecimal string
    match i64::from_str_radix(&hex, 16) {
        Ok(value) => Ok(Value::Int(value)),
        Err(e) => Err(format!("Failed to parse hexadecimal string: {}", e)),
    }
}
