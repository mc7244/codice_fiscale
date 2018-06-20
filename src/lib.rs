// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This crate provide tools to manage the Italian *codice fiscale*, which
//! (for anyone who doesn't live in Italy) is a code associated to every
//! individual which helps with identification in public services.
//! 
//! We currently provide codice fiscale calculation. Check of the codice
//! will be the next feature.
//! 
//! For anyone interested, here's an explanation (Italian language) on how
//! the codice fiscale is calculated:
//! https://it.wikipedia.org/wiki/Codice_fiscale#Generazione_del_codice_fiscale

#![recursion_limit = "1024"] // For error_chain

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
    /// Birthdate must be a valid YYYY-MM-AA date
    pub birthdate: String,
    pub gender: Gender,
    /// Belfiore codice for comune (ie E889). You must know it for now;
    /// we may provide a database in the future
    pub comune: String,
}

#[derive(Debug, Clone, PartialEq)]
struct CodiceFiscaleParts {
    surname     : String,
    name        : String,
    birthyear   : String,
    birthmonth  : char,
    birthday    : String,
    birthdate   : String,
    comune      : String,
    checkchar   : char,
}

/// The real thing: codice fiscale calculation
#[derive(Debug, PartialEq)]
pub struct CodiceFiscale {
    persondata    : PersonData,
    codice        : String,
    codice_parts  : CodiceFiscaleParts,
}

static CONSONANTS   : &str = "BCDFGHJKLMNPQRSTVWXYZ";
static VOWELS       : &str = "AEIOU";
static MONTHLETTERS : [char; 12] = ['A', 'B', 'C', 'D', 'E', 'H', 'L', 'M', 'P', 'R', 'S', 'T'];
static PAT_COMUNE   : &str = r"\w\d\d\d";
static CHECKMODULI  : [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
    'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

impl CodiceFiscale {
    /// Constructor which creates a new CodiceFiscale struct from personal data,
    /// which has to be provided as a PersonData struct
    pub fn new(initdata: &PersonData) -> Result<CodiceFiscale>  {
        let mut codice_fiscale = CodiceFiscale {
            persondata      : initdata.clone(),
            codice          : "".to_string(),
            codice_parts    : CodiceFiscaleParts {
                surname     : "".to_string(),
                name        : "".to_string(),
                birthyear   : "".to_string(),
                birthmonth  : '_',
                birthday    : "".to_string(),
                birthdate   : "".to_string(),
                comune      : "".to_string(),
                checkchar   : '_',
            }
        };

        let mut codice = "".to_string();
        codice.push_str( codice_fiscale.calc_surname() );
        codice.push_str( codice_fiscale.calc_name() );
        codice.push_str( codice_fiscale.calc_birthdate()? );
        codice.push_str( codice_fiscale.calc_comune()? );
        codice_fiscale.codice = codice.clone();
        codice.push( codice_fiscale.calc_checkchar() );

        codice_fiscale.codice = codice;
        Ok(codice_fiscale)
    }

    pub fn codice(&self) -> &str {
        &self.codice
    }

    pub fn persondata(&self) -> &PersonData {
        &self.persondata
    }

    // SURNAME
    fn calc_surname(&mut self) -> &str {
        let mut surname_consonants = String::new();
        let mut surname_vowels = String::new();
        for ch in self.persondata.surname.to_uppercase().chars() {
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

        self.codice_parts.surname = cf_surname;
        &self.codice_parts.surname
    }

    // NAME
    fn calc_name(&mut self) -> &str {
        let mut name_consonants = String::new();
        let mut name_vowels = String::new();
        for ch in self.persondata.name.to_uppercase().chars() {
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

        self.codice_parts.name = cf_name;
        &self.codice_parts.name
    }

    fn calc_birthdate(&mut self) -> Result<&str> {
       // BIRTHDATE
        let tm_birthdate : Tm;
        match time::strptime(&self.persondata.birthdate, "%Y-%m-%d") {
            Ok(v)   => tm_birthdate = v,
            Err(_e) => return Err(ErrorKind::InvalidBirthdate.into())
        };
        let tm_year = tm_birthdate.tm_year;
        self.codice_parts.birthyear =
            if tm_year < 100 { tm_year } else { tm_year - 100 }.to_string();
        self.codice_parts.birthmonth = MONTHLETTERS[tm_birthdate.tm_mon as usize];
        self.codice_parts.birthday = format!("{:02}",
            if self.persondata.gender == Gender::F { 40+tm_birthdate.tm_mday } else { tm_birthdate.tm_mday }
        );

        self.codice_parts.birthdate.push_str(&self.codice_parts.birthyear);
        self.codice_parts.birthdate.push(self.codice_parts.birthmonth);
        self.codice_parts.birthdate.push_str(&self.codice_parts.birthday);
        Ok(&self.codice_parts.birthdate)
    }

    fn calc_comune(&mut self) -> Result<&str> {
        let rxc_comune = Regex::new(PAT_COMUNE).expect("Regex init error");
        if !rxc_comune.is_match(&self.persondata.comune)  {
            return Err(ErrorKind::InvalidComune.into());
        }

        self.codice_parts.comune = self.persondata.comune.to_uppercase();
        Ok(&self.codice_parts.comune)
    }

    // CHECK CHAR
    fn calc_checkchar(&mut self) -> char {
        let mut odd_sum : usize = 0;
        let mut even_sum : usize = 0;
        for chi in self.codice.char_indices() {
            if chi.0 % 2 == 0 { odd_sum += cfstatics::CHECKCHARS[&chi.1].0 }
            else { even_sum += cfstatics::CHECKCHARS[&chi.1].1 }
        }
        let checkidx : usize = (odd_sum + even_sum) % 26;

        self.codice_parts.checkchar = CHECKMODULI[checkidx];
        self.codice_parts.checkchar
    }
}

