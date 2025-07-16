// Compare two characters for equality
pub fn eq(a: char, b: char) -> bool {
    a == b
}

// Add three f32 numbers
pub fn add(x: f32, y: f32, z: f32) -> f32 {
    x + y + z
}

// Cast(convert) all values to f32 before adding
pub fn cast(x: u8, y: i8, z: f32) -> f32 {
    x as f32 + y as f32 + z
}
