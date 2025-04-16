pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut map = [0; 26];
    for byte in magazine.as_bytes() {
        map[*byte as usize - 97] += 1;
    }

    for byte in ransom_note.as_bytes() {
        map[*byte as usize - 97] -= 1;
        if map[*byte as usize - 97] < 0 {
            return false;
        }
    }

    true
}
