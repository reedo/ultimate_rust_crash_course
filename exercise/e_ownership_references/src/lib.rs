/// Prints a message indicating if the word is singular or plural
///
/// # Arguments
///
/// * `word` - A string which is a single English word
pub fn inspect(word: &String) {
    if word.to_lowercase().ends_with("s") {
        println!("{} is a plural", word);
    } else {
        println!("{} is singular", word);
    }
}

/// Pluralises the given word
///
/// # Arguments
///
/// * `word` - A string which is a single English word
pub fn change(word: &mut String) {
    if !word.ends_with("s") {
        word.push_str("s")
    }
}

/// Consumes the given word
///
/// # Arguments
///
/// * `word` - A string which is a single English word
pub fn eat(word: String) -> bool {
    word.starts_with("b") && word.contains("a")
}

/// Adds two numbers together and returns the result
///
/// # Arguments
///
/// * `a` - The first number
/// * `b` - The second number
pub fn add(a: &i32, b: &i32) -> i32 {
    a + b
}
