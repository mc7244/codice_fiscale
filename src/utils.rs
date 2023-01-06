static CONSONANTS: &str = "BCDFGHJKLMNPQRSTVWXYZ";
static VOWELS: &str = "AEIOU";

fn is_vowel(c: &char) -> bool {
    VOWELS.contains(*c)
}

fn is_consonant(c: &char) -> bool {
    CONSONANTS.contains(*c)
}

fn extract_consonants(name: &str) -> String {
    name.to_uppercase()
        .chars()
        .filter(|x| is_vowel(x))
        .rev()
        .collect()
}

pub fn calc_name_component(name: &str) -> String {
    let name = name.to_uppercase();
    let consonants = name.chars().filter(is_consonant);
    let mut consonants: String = if consonants.count() <= 3 {
        name.chars().filter(is_consonant).take(3).collect()
    } else {
        name.chars()
            .filter(is_consonant)
            .enumerate()
            .filter(|x| x.0 != 1)
            .map(|x| x.1)
            .take(3)
            .collect()
    };

    if consonants.len() < 3 {
        consonants += &name
            .chars()
            .filter(is_vowel)
            .take(3 - consonants.len())
            .collect::<String>();
    }
    if consonants.len() < 3 {
        consonants += &['X']
            .iter()
            .cycle()
            .take(3 - consonants.len())
            .collect::<String>();
    }
    dbg!(consonants)
}

pub fn calc_surname_component(name: &str) -> String {
    let part_consonants: String = name
        .to_uppercase()
        .chars()
        .filter(|x| is_consonant(x))
        .collect();
    let mut part_vowels = extract_consonants(name);
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
