use crate::value::Value;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Generate a random seed string of a given length
/// Example: generate(10) => "a1b2c3d4e5"
pub fn generate(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Seed.generate requires exactly 1 argument: length".to_string());
    }
    
    let length = match &args[0] {
        Value::Int(n) => *n as usize,
        Value::Float(n) => *n as usize,
        _ => return Err(format!("Argument to generate must be a number, got {:?}", args[0])),
    };
    
    if length == 0 {
        return Ok(Value::String(String::new()));
    }
    
    let charset: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = rand::thread_rng();
    
    let seed: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx] as char
        })
        .collect();
    
    Ok(Value::String(seed))
}

/// Create a 2D map from a seed string
/// Example: map("razen123", 3, 3) => [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
pub fn map_seed(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 3 {
        return Err("Seed.map requires exactly 3 arguments: seed, width, height".to_string());
    }
    
    let seed_str = args[0].as_string()?;
    
    let width = match &args[1] {
        Value::Int(n) => *n as usize,
        Value::Float(n) => *n as usize,
        _ => return Err(format!("Second argument to map must be a number, got {:?}", args[1])),
    };
    
    let height = match &args[2] {
        Value::Int(n) => *n as usize,
        Value::Float(n) => *n as usize,
        _ => return Err(format!("Third argument to map must be a number, got {:?}", args[2])),
    };
    
    if width == 0 || height == 0 {
        return Ok(Value::Array(Vec::new()));
    }
    
    // Create a deterministic RNG from the seed string
    let seed = hash_string(&seed_str);
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    
    // Generate the 2D map
    let mut map = Vec::new();
    for _ in 0..height {
        let mut row = Vec::new();
        for _ in 0..width {
            row.push(Value::Int(rng.gen_range(0..10)));
        }
        map.push(Value::Array(row));
    }
    
    Ok(Value::Array(map))
}

/// Generate a noise map using Perlin noise
/// Example: noise_map("razen123", 5, 5, 0.5) => [[0.1, 0.2, ...], [...], ...]
pub fn noise_map(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 4 {
        return Err("Seed.noise_map requires exactly 4 arguments: seed, width, height, scale".to_string());
    }
    
    let seed_str = args[0].as_string()?;
    
    let width = match &args[1] {
        Value::Int(n) => *n as usize,
        Value::Float(n) => *n as usize,
        _ => return Err(format!("Second argument to noise_map must be a number, got {:?}", args[1])),
    };
    
    let height = match &args[2] {
        Value::Int(n) => *n as usize,
        Value::Float(n) => *n as usize,
        _ => return Err(format!("Third argument to noise_map must be a number, got {:?}", args[2])),
    };
    
    let scale = args[3].as_float()?;
    
    if width == 0 || height == 0 {
        return Ok(Value::Array(Vec::new()));
    }
    
    // Create a deterministic RNG from the seed string
    let seed = hash_string(&seed_str);
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    
    // Generate a simple noise map (this is a simplified version without actual Perlin noise)
    let mut map = Vec::new();
    for y in 0..height {
        let mut row = Vec::new();
        for x in 0..width {
            // Simple noise function based on coordinates and seed
            let value = (rng.gen::<f64>() + 
                        (x as f64 / width as f64) * scale + 
                        (y as f64 / height as f64) * scale) % 1.0;
            row.push(Value::Float(value));
        }
        map.push(Value::Array(row));
    }
    
    Ok(Value::Array(map))
}

/// Generate a random name based on a seed
/// Example: name("player123") => "Brave Warrior"
pub fn name(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("Seed.name requires exactly 1 argument: seed".to_string());
    }
    
    let seed_str = args[0].as_string()?;
    
    // Create a deterministic RNG from the seed string
    let seed = hash_string(&seed_str);
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    
    // Lists of adjectives and nouns for name generation
    let adjectives = [
        "Brave", "Mighty", "Wise", "Swift", "Clever", "Ancient", "Mystic", 
        "Noble", "Fierce", "Gentle", "Wild", "Silent", "Golden", "Silver"
    ];
    
    let nouns = [
        "Warrior", "Mage", "Hunter", "Guardian", "Knight", "Scholar", "Ranger",
        "Healer", "Druid", "Paladin", "Rogue", "Bard", "Wizard", "Sorcerer"
    ];
    
    // Select a random adjective and noun
    let adjective = adjectives[rng.gen_range(0..adjectives.len())];
    let noun = nouns[rng.gen_range(0..nouns.len())];
    
    // Combine them to form a name
    let name = format!("{} {}", adjective, noun);
    
    Ok(Value::String(name))
}

/// Helper function to hash a string to a u64
fn hash_string(s: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}
