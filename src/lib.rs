// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This crate provide tools to manage the Italian *codice fiscale*, which
//! (for anyone who doesn't live in Italy) is a code associated to every
//! individual which helps with identification in public services.

#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

extern crate regex;

use regex::Regex;

mod errors {
    error_chain! {
        errors {
            InvalidComune {
                description("invalid-comune")
                display("Invalid comune code, should be something like E889 - you supplied")
            }
        }
    }
}
use errors::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Gender {
    M, F
}

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
}

static PAT_COMUNE : &str = r"\w\d\d\d";

impl CodiceFiscale {
    pub fn new(initdata: &PersonData) -> Result<CodiceFiscale>  {
        let rxc_comune = Regex::new(PAT_COMUNE).expect("Regex init error");
        if !rxc_comune.is_match(&initdata.comune)  {
            return Err(ErrorKind::InvalidComune.into());
            //bail!("Invalid comune code, should be something like E889".to_string());
            //return Err(CFError {
            //     message: "Invalid comune code, should be something like E889".to_string(),
            // } );
        }
        Ok(CodiceFiscale {
            persondata: initdata.clone()
            // persondata : PersonData {
                // name: initdata.name.clone(),
                // surname: initdata.surname.clone(),
                // birthdate: initdata.birthdate.clone(),
                // gender: initdata.gender.clone(),
                // comune: initdata.comune.clone(),
            //}
        })
    }

}

