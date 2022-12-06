fn main() {
    let chars: Vec<char> = include_str!("input.txt").trim().chars().collect();

    // Set to 14 for part 2
    let char_len = 4;

    for i in 0..chars.len() {
        let range = chars[i..i + char_len].to_vec();

        if !(1..range.len()).any(|i| range[i..].contains(&range[i - 1])) {
            println!("{}", i + char_len);
            break;
        }
    }
}
