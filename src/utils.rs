static CONSONANTS: &str = "BCDFGHJKLMNPQRSTVWXYZ";
static VOWELS: &str = "AEIOU";

fn is_vowel(c: &char) -> bool {
    VOWELS.contains(*c)
}

fn is_consonant(c: &char) -> bool {
    CONSONANTS.contains(*c)
}

fn extract_consonants(name: &str) -> String {
    name.to_uppercase().chars()
        .filter(|x| is_vowel(x))
        .rev()
        .collect()
}

/// Remove the 2nd consonant
pub fn prepare_name(name: &str) -> String {
    if extract_consonants(name).len() > 2 {
        let mut res = name.to_uppercase();
        let mut count: (u8, usize) = (0,0);
        for (i, ch) in res.chars().enumerate() {
            if is_consonant(&ch) {
                count.0 += 1;
                count.1 = i;
            }
            if count.0 == 2 {
                break;
            }
        }
        res.remove(count.1);
        res
    } else {
        name.to_owned()
    }
}

pub fn calc_name_component(name: &str) -> String {
    let part_consonants: String = name.to_uppercase().chars()
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