pub fn zeros() -> [u32; 100] {
    [0; 100] // creates an array with 100 elements, all set to 0
}

pub fn first_3(s: &[u32]) -> &[u32] {
    &s[..3] // slice from the beginning up to (but not including) index 3
}

pub fn last_3(s: &[u32]) -> &[u32] {
    &s[s.len() - 3..] // slice from the third-to-last to the end
}
