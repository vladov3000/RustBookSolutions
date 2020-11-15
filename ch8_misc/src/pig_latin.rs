const VOWELS: [char; 6] = ['a', 'e', 'i', 'y', 'o', 'u'];

fn word_to_pl(word: &str) -> String {
    let mut first = word.chars().nth(0).unwrap();
    first.make_ascii_lowercase();

    let mut res = String::new();

    if VOWELS.contains(&first) {
        res.push_str(word);
        res.push_str("-hay")
    } else {
        res.push_str(&word[1..]);
        res.push('-');
        res.push(first);
        res.push_str("ay");
    }

    res
}

pub fn text_to_pl(text: &str) -> String {
    let mut res = String::new();
    for word in text.split_whitespace() {
        let pig = word_to_pl(word);
        res.push_str(&pig);
        res.push(' ');
    }
    res
}
