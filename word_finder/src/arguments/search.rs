pub fn search_word(line: &Vec<String>, word: &str) {
    let mut x: i32 = 0;
    for (i, l) in line.iter().enumerate() {
        if l.contains(word) {
            println!("{}: {}", i + 1, l);
            x += 1;
        }
    }
    if x == 0 {
        println!("No lines found with the word: {}", word);
    }
}
