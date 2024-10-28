fn main() {
    //read the file hamming_tests.txt
    let contents = std::fs::read_to_string("hamming_tests.txt").expect("Could not read file");
    //split the contents of the file by new line
    let lines: Vec<&str> = contents.split("\n").collect();
    //iterate over the lines
    for line in lines {
        //split the line by space and collect the words in a vector of strings
        let words: Vec<&str> = line.split(" ").collect();
        //trim the words to remove the carriage return character if present
        let words: Vec<&str> = words
            .iter()
            .map(|&word| word.trim_end_matches('\r'))
            .collect();
        //pass the words to the hamming_distance function and store the result
        let result = hamming_distance(words[0], words[1]);

        //print the result
        match result {
            Some(value) =>
                println!("Hamming distance between {} and {} is {}", words[0], words[1], value),
            None => println!("The two strings have different lengths"),
        }
    }
}

fn hamming_distance(s1: &str, s2: &str) -> Option<u32> {
    //check if the length of the two strings are equal
    if s1.len() != s2.len() {
        return None;
    }
    //initialize a counter to count the number of differences
    let mut count = 0;
    //iterate over the characters of the two strings
    for (c1, c2) in s1.chars().zip(s2.chars()) {
        //increment the counter if the characters are different
        if c1 != c2 {
            count += 1;
        }
    }
    //return the count
    Some(count)
}
