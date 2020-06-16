static CONSONANTS: &str = "BCDFGHJKLMNPQRSTVWXYZ";
static VOWELS: &str = "AEIOU";

fn is_vowel(c: &char) -> bool {
    VOWELS.contains(*c)
}

fn is_consonant(c: &char) -> bool {
    CONSONANTS.contains(*c)
}

pub fn calc_name_component(name: &str) -> String {
    let part_consonants: String = name.to_uppercase().chars()
        .filter(|x| is_consonant(x))
        .collect();
    let mut part_vowels: String = name.to_uppercase().chars()
        .filter(|x| is_vowel(x))
        .rev()
        .collect();
    let mut cf_part = String::new();
    cf_part.push_str(part_consonants.chars().take(3).collect::<String>().as_ref());
    // Push vowels if needed (and there are)
    while cf_part.len() < 3 && !part_vowels.is_empty() {
        cf_part.push(part_vowels.pop().unwrap());
    }
    // Push Xs for missing chars
    while cf_part.len() < 3 {
        cf_part.push('X');
    }
    cf_part
}