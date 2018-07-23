// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This crate provides tools to manage the Italian *codice fiscale*, which
//! (for anyone who doesn't live in Italy) is a code associated to every
//! individual which helps with identification in public services.
//! 
//! We currently provide codice fiscale calculation. Check of the codice
//! will be the next feature.
//! 
//! For anyone interested, here's an explanation (Italian language) on how
//! the codice fiscale is calculated:
//! https://it.wikipedia.org/wiki/Codice_fiscale#Generazione_del_codice_fiscale

extern crate time;
extern crate regex;

#[macro_use] extern crate failure;
#[macro_use] extern crate lazy_static;

use time::Tm;
use regex::Regex;
use failure::Error;
use std::collections::HashMap;

// #[derive(Debug, Fail)]
// enum CodiceFiscaleError {
//     #[fail(display = "Invalid Belfiore code: {}, should be something like E889", belfiore)]
//     InvalidComune {
//         belfiore: String,
//     },
//     #[fail(display = "Invalid birthdate: {}, please provide a YYYY-MM-DD format date", birthdate)]
//     InvalidBirthdate {
//         birthdate: String,
//     },
//     #[fail(display = "The length of a codice fiscale must be 16 characters", removeme)]
//     InvalidCodiceLen {
//         removeme: String,
//     },
//     #[fail(display = "The check char for this codice is not correct, so the codice is not a valid one", removeme)]
//     InvalidCodiceCheckChar {
//         removeme: String,
//     }
// }

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
    pub name        : String,
    pub surname     : String,
    /// Birthdate must be a valid YYYY-MM-AA date
    pub birthdate   : String,
    pub gender      : Gender,
    /// Belfiore codice for comune (ie E889). You must know it for now;
    /// we may provide a database in the future
    pub belfiore    : String,
}

#[derive(Debug, Clone, PartialEq)]
struct CodiceFiscaleParts {
    surname     : String,
    name        : String,
    birthyear   : String,
    birthmonth  : char,
    birthday    : String,
    birthdate   : String,
    belfiore    : String,
    checkchar   : char,
}

/// codice fiscale calculation and parsing
/// Note: the PartialEq trait here supposes every PersonData and CodiceFiscaleParts match perfectly,
/// which actually makes for identical persons and not only identical code.
/// For comparison you might be better just comparing what is returned by codice() method
#[derive(Debug, PartialEq)]
pub struct CodiceFiscale {
    persondata    : PersonData,
    codice        : String,
    codice_parts  : CodiceFiscaleParts,
}

static CONSONANTS   : &str = "BCDFGHJKLMNPQRSTVWXYZ";
static VOWELS       : &str = "AEIOU";
static CENTURY_BASE : i32 = 2000; // This will need to be changed in 2100
static MONTHLETTERS : [char; 12] = ['A', 'B', 'C', 'D', 'E', 'H', 'L', 'M', 'P', 'R', 'S', 'T'];
static PAT_BELFIORE : &str = r"\w\d\d\d";
static CHECKMODULI  : [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
    'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];
lazy_static! {
    static ref CHECKCHARS: HashMap<char, (u8, u8)> = {
        let mut m = HashMap::new();
        m.insert('A', (1, 0));
        m.insert('B', (0, 1));
        m.insert('C', (5, 2));
        m.insert('D', (7, 3));
        m.insert('E', (9, 4));
        m.insert('F', (13, 5));
        m.insert('G', (15, 6));
        m.insert('H', (17, 7));
        m.insert('I', (19, 8));
        m.insert('J', (21, 9));
        m.insert('K', (2, 10));
        m.insert('L', (4, 11));
        m.insert('M', (18, 12));
        m.insert('N', (20, 13));
        m.insert('O', (11, 14));
        m.insert('P', (3, 15));
        m.insert('Q', (6, 16));
        m.insert('R', (8, 17));
        m.insert('S', (12, 18));
        m.insert('T', (14, 19));
        m.insert('U', (16, 20));
        m.insert('V', (10, 21));
        m.insert('W', (22, 22));
        m.insert('X', (25, 23));
        m.insert('Y', (24, 24));
        m.insert('Z', (23, 25));
        m.insert('0', (1, 0));
        m.insert('1', (0, 1));
        m.insert('2', (5, 2));
        m.insert('3', (7, 3));
        m.insert('4', (9, 4));
        m.insert('5', (13, 5));
        m.insert('6', (15, 6));
        m.insert('7', (17, 7));
        m.insert('8', (19, 8));
        m.insert('9', (21, 9));
        m
    };
}

impl CodiceFiscale {
    /// Constructor which creates a CodiceFiscale struct from personal data,
    /// which has to be provided as a PersonData struct
    pub fn new(initdata: &PersonData) -> Result<CodiceFiscale, Error>  {
        let mut cf = CodiceFiscale {
            persondata      : initdata.clone(),
            codice          : "".to_string(),
            codice_parts    : CodiceFiscaleParts {
                surname     : "".to_string(),
                name        : "".to_string(),
                birthyear   : "".to_string(),
                birthmonth  : '_',
                birthday    : "".to_string(),
                birthdate   : "".to_string(),
                belfiore    : "".to_string(),
                checkchar   : '_',
            }
        };

        let mut codice = "".to_string();
        codice.push_str( cf.calc_surname() );
        codice.push_str( cf.calc_name() );
        codice.push_str( cf.calc_birthdate()? );
        codice.push_str( cf.calc_belfiore()? );
        cf.codice = codice.clone();
        codice.push( cf.calc_checkchar() );

        cf.codice = codice;
        Ok(cf)
    }

    /// Constructor which creates a CodiceFiscale struct from a codice fiscale string
    pub fn parse(codice: &str) -> Result<CodiceFiscale, Error>  {
        let mut cf = CodiceFiscale {
            persondata      : PersonData {
                name        : "".to_string(),
                surname     : "".to_string(),
                birthdate   : "".to_string(),
                gender      : Gender::M,
                belfiore    : "".to_string(),
            },
            codice          : "".to_string(),
            codice_parts    : CodiceFiscaleParts {
                surname     : "".to_string(),
                name        : "".to_string(),
                birthyear   : "".to_string(),
                birthmonth  : '_',
                birthday    : "".to_string(),
                birthdate   : "".to_string(),
                belfiore     : "".to_string(),
                checkchar   : '_',
            }
        };

        // First off, validate CF to see if it's a valid Code
        if codice.len() != 16 {
            bail!("invalid-length");
        }

        // The let's see if the check char we calculate matches
        let mut codice_nolast = codice.to_uppercase().to_string();
        let codice_checkchar = match codice_nolast.pop() {
            Some(cc)    => cc,
            None        => bail!("invalid-checkchar")
        }; 
        cf.codice = codice_nolast.to_string();
        if cf.calc_checkchar() != codice_checkchar {
            bail!("invalid-checkchar");
        }

        cf.codice_parts.surname = codice[0..3].to_string();
        if !Regex::new("^[A-Z]{3}$").unwrap().is_match(&cf.codice_parts.surname) {
            bail!("invalid-surname");
        }
        cf.persondata.surname = cf.codice_parts.surname.clone();

        cf.codice_parts.name = codice[3..6].to_string();
        if !Regex::new("^[A-Z]{3}$").unwrap().is_match(&cf.codice_parts.name) {
            bail!("invalid-name");
        }
        cf.persondata.name = cf.codice_parts.name.clone();

        // It is impossible to day with certainity to which century a 2-digits year belongs. So we suppose that if it's // in the future compared to now, it's in this century, otherwise in the past one
        // (this has implications only for parsing, not for validation, unless we stump into and unexisting Feb29)
        cf.codice_parts.birthyear = codice[6..8].to_string();
        let birthyear_num
            = CENTURY_BASE + i32::from_str_radix(&cf.codice_parts.birthyear, 10).expect("invalid-birthyear");
        let tm_now_year = time::now_utc().tm_year + 1900;
        let birthyear = if tm_now_year > birthyear_num { birthyear_num } else { birthyear_num - 100 };

        cf.codice_parts.birthmonth = codice.chars().nth(8).unwrap();
        cf.codice_parts.birthday = codice[9..11].to_string();
        let mut birthdate : String =  format!("{:04}", birthyear);
        birthdate.push('-');
        let birthmonth = MONTHLETTERS.binary_search(&cf.codice_parts.birthmonth).expect("invalid-birthmonth");
        birthdate.push_str( &format!("{:02}", (birthmonth + 1)) );
        birthdate.push('-');
        birthdate.push_str(&cf.codice_parts.birthday);
        match time::strptime(&birthdate, "%Y-%m-%d") {
            Ok(_v)  => cf.persondata.birthdate = birthdate,
            Err(_e) => bail!("invalid-birthdate".to_string() + &birthdate)
        };

        cf.codice_parts.belfiore = codice[11..15].to_string();
        let rxc_belfiore = Regex::new(PAT_BELFIORE).expect("Regex init error");
        if !rxc_belfiore.is_match(&cf.codice_parts.belfiore)  {
            bail!("invalid-belfiore-code");
        }
        cf.persondata.belfiore = cf.codice_parts.belfiore.clone();

        cf.codice.push(codice_checkchar);
        Ok(cf)
    }

    /// Static method returns true if codice fiscale is valid, false otherwise.
    /// This is the method almost everybody will use
    pub fn check(codice: &str) -> bool  {
        match CodiceFiscale::parse(codice) {
            Ok(_cf)  => true,
            Err(_e)  => false
        }
    }

    /// Returns the codice
    pub fn codice(&self) -> &str {
        &self.codice
    }

    /// Returns the person data
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

    fn calc_birthdate(&mut self) -> Result<&str, Error> {
        // BIRTHDATE
        let tm_birthdate : Tm;
        match time::strptime(&self.persondata.birthdate, "%Y-%m-%d") {
            Ok(v)   => tm_birthdate = v,
            Err(_e) => bail!("invalid-birthdate")
        };
        let tm_year = tm_birthdate.tm_year + 1900;
        self.codice_parts.birthyear =
            if tm_year < CENTURY_BASE { tm_year - 1900 } else { tm_year - CENTURY_BASE }.to_string();
        self.codice_parts.birthmonth = MONTHLETTERS[tm_birthdate.tm_mon as usize];
        self.codice_parts.birthday = format!("{:02}",
            if self.persondata.gender == Gender::F { 40+tm_birthdate.tm_mday } else { tm_birthdate.tm_mday }
        );
        self.codice_parts.birthdate.push_str(&self.codice_parts.birthyear);
        self.codice_parts.birthdate.push(self.codice_parts.birthmonth);
        self.codice_parts.birthdate.push_str(&self.codice_parts.birthday);
        Ok(&self.codice_parts.birthdate)
    }

    fn calc_belfiore(&mut self) -> Result<&str, Error> {
        let rxc_belfiore = Regex::new(PAT_BELFIORE).expect("Regex init error");
        if !rxc_belfiore.is_match(&self.persondata.belfiore)  {
            bail!("invalid-belfiore-code");
        }

        self.codice_parts.belfiore = self.persondata.belfiore.to_uppercase();
        Ok(&self.codice_parts.belfiore)
    }

    // CHECK CHAR
    fn calc_checkchar(&mut self) -> char {
        let mut checksum : u8 = 0;


        for chi in self.codice.char_indices() {
            if chi.0 % 2 == 0 { checksum += CHECKCHARS[&chi.1].0 }
            else { checksum += CHECKCHARS[&chi.1].1 }
        }

        self.codice_parts.checkchar = CHECKMODULI[(checksum % 26) as usize];
        self.codice_parts.checkchar
    }
}

