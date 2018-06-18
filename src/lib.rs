mod error;

use error::*;

pub enum Sex {
    M, F
}

pub struct PersonData {
    pub name: String,
    pub surname: String,
    pub birthdate: String,
    pub sex: Sex,
    pub comune: String,
}

pub fn calculate(_persondata: &PersonData) -> Result<String, CFError>  {
    Ok( "CIAO".to_string() )
}

