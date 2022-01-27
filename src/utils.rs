pub fn str_unique_by_characters(s: &str) -> bool {
    s.chars()
        .enumerate()
        .find_map(|(i, c)| {
            s.chars()
                .enumerate()
                .skip(i + 1)
                .find(|(_, other)| c == *other)
                .map(|(j, _)| (i, j, c))
        })
        .is_none()
}

pub fn str_to_five_char(s: &str) -> [char; 5] {
    s.chars()
        .take(5)
        .enumerate()
        .fold([' '; 5], |mut arr, (i, c)| {
            arr[i] = c;
            arr
        })
}
