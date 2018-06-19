// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This crate provide tools to manage the Italian *codice fiscale*, which
//! (for anyone who doesn't live in Italy) is a code associated to every
//! individual which helps with identification in public services.

#![recursion_limit = "1024"]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate error_chain;
extern crate time;
extern crate regex;

use time::Tm;
use regex::Regex;

mod errors {
    error_chain! {
        errors {
            InvalidComune {
                description("invalid-comune")
                display("Invalid comune code, should be something like E889")
            }
            InvalidBirthdate {
                description("invalid-birthdate")
                display("Invalid birthdate, please provide a YYYY-MM-DD format date")
            }
        }
    }
}
use errors::*;

mod cfstatics;
//use cfstatics::*;

/// Gender enum to specify gender in PersonData struct
/// Italian government only accepts either male or female!
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Gender {
    M, F
}

/// PersonData struct to pass to new() constructor for calculation of
/// codice fiscale
#[derive(Debug, Clone, PartialEq)]
pub struct PersonData {
    pub name: String,
    pub surname: String,
    pub birthdate: String,
    pub gender: Gender,
    pub comune: String,
}

#[derive(Debug, PartialEq)]
pub struct CodiceFiscale {
    pub persondata: PersonData,
    pub codice    : String,
}

static CONSONANTS   : &str = "BCDFGHJKLMNPQRSTVWXYZ";
static VOWELS       : &str = "AEIOU";
static MONTHLETTERS : [char; 12] = ['A', 'B', 'C', 'D', 'E', 'H', 'L', 'M', 'P', 'R', 'S', 'T'];
static PAT_COMUNE   : &str = r"\w\d\d\d";
static CHECKMODULI  : [char; 26s] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
    'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

 /// Constructor which creates a new CodiceFiscale struct from personal data,
 /// which has to be provided as a PersonData struct
impl CodiceFiscale {
    pub fn new(initdata: &PersonData) -> Result<CodiceFiscale>  {
        let mut codice = "".to_string();

        // SURNAME
        let mut surname_consonants = String::new();
        let mut surname_vowels = String::new();
        for ch in initdata.surname.to_uppercase().chars() {
            //println!("{}", ch);
            if CONSONANTS.contains(ch) {
                surname_consonants.push(ch);
            } else if VOWELS.contains(ch) {
                surname_vowels.push(ch);
            }
        }
        let mut cf_surname = String::new();
        if surname_consonants.len() > 3 {
            cf_surname.push_str(&surname_consonants[..3]);
        } else {
            cf_surname.push_str(&surname_consonants);
        }
        // Push vowels if needed (and there are)
        while cf_surname.len() < 3 && surname_vowels.len() > 0 {
            cf_surname.push(surname_vowels.remove(0));
        }
        // Push Xs for missing chars
        while cf_surname.len() < 3 {
            cf_surname.push('X');
        }
        codice.push_str(&cf_surname);

        // NAME
        let mut name_consonants = String::new();
        let mut name_vowels = String::new();
        for ch in initdata.name.to_uppercase().chars() {
            //println!("{}", ch);
            if CONSONANTS.contains(ch) {
                name_consonants.push(ch);
            } else if VOWELS.contains(ch) {
                name_vowels.push(ch);
            }
        }
        let mut cf_name = String::new();
        if name_consonants.len() > 3 {
            cf_name.push_str(&name_consonants[..1]);
            cf_name.push_str(&name_consonants[2..4]);
        } else {
            cf_name.push_str(&name_consonants);
            // Push vowels if needed (and there are)
            while cf_name.len() < 3 && name_vowels.len() > 0 {
                cf_name.push(name_vowels.remove(0));
            }
            // Push Xs for missing chars
            while cf_name.len() < 3 {
                cf_name.push('X');
            }
        }
        codice.push_str(&cf_name);

        // BIRTHDATE
        let tm_birthdate : Tm;
        match time::strptime(&initdata.birthdate, "%Y-%m-%d") {
            Ok(v)   => tm_birthdate = v,
            Err(_e) => return Err(ErrorKind::InvalidBirthdate.into())
        };
        let tm_year = tm_birthdate.tm_year;
        codice.push_str(&
            if tm_year < 100 { tm_year } else { tm_year - 100 }.to_string()
        );
        codice.push( MONTHLETTERS[tm_birthdate.tm_mon as usize] );
        codice.push_str( &format!("{:02}",
            if initdata.gender == Gender::F { 40+tm_birthdate.tm_mday } else { tm_birthdate.tm_mday }
        ) );

        let rxc_comune = Regex::new(PAT_COMUNE).expect("Regex init error");
        if !rxc_comune.is_match(&initdata.comune)  {
            return Err(ErrorKind::InvalidComune.into());
        }
        codice.push_str(&initdata.comune.to_uppercase());

        // CHECK DIGIT
        let mut odd_sum : usize = 0;
        let mut even_sum : usize = 0;
        for chi in codice.char_indices() {
            if chi.0 % 2 == 0 { odd_sum += cfstatics::CHARTABLE[&chi.1].0 }
            else { even_sum += cfstatics::CHARTABLE[&chi.1].1 }
        }
        let chekidx : usize = (odd_sum + even_sum) % 26;
        codice.push(CHECKMODULI[checkidx]);

        Ok(CodiceFiscale {
            persondata: initdata.clone(),
            codice    : codice,
        })
    }
}

