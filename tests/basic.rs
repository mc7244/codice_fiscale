#![cfg(test)]
extern crate codice_fiscale;
use codice_fiscale::*;

const TEST_CF_OK            : &str = "BLTMHL77S04E889G";
const TEST_CF_ERR_CHECKCHAR : &str = "BLTMHL77S04E889Y";
const TEST_BELFIORE         : &str = "E889";

fn make_new_test_persondata() -> PersonData {
    PersonData {
        name        : "Michele".to_string(),
        surname     : "Beltrame".to_string(),
        birthdate   : "1977-11-04".to_string(),
        gender      : Gender::M,
        belfiore    : TEST_BELFIORE.to_string(),
    }
}

fn make_parse_test_persondata() -> PersonData {
    PersonData {
        name        : "MHL".to_string(),
        surname     : "BLT".to_string(),
        birthdate   : "1977-11-04".to_string(),
        gender      : Gender::M,
        belfiore    : TEST_BELFIORE.to_string(),
    }
}

#[test]
fn t_new() {
    let persondata = make_new_test_persondata();
    assert_eq!(
        CodiceFiscale::new(&persondata).unwrap().codice(),
        TEST_CF_OK
    );
}

#[test]
fn t_new_err_belfiore() {
    let mut persondata = make_new_test_persondata();
    persondata.belfiore = "EX".to_string();
    assert_eq!(
        format!("{}",CodiceFiscale::new(&persondata).err().unwrap()),
        "invalid-belfiore-code"
    );
}

#[test]
fn t_new_err_birthdate() {
    let mut persondata = make_new_test_persondata();
    persondata.birthdate = "1977-04-32".to_string();
    assert_eq!(
        format!("{}",CodiceFiscale::new(&persondata).err().unwrap()),
        "invalid-birthdate"
    );
}

#[test]
fn t_scoping() {
    let cf;
    {
        let pdata = make_new_test_persondata();
        cf = CodiceFiscale::new(&pdata).unwrap();
    }
    assert_eq!(cf.persondata().belfiore, "E889");
}

#[test]
fn t_parse_ok() {
    let cf = CodiceFiscale::parse(TEST_CF_OK).unwrap();
    let persondata = make_parse_test_persondata();
    assert_eq!(cf.persondata(), &persondata);
    // TODO: check whole persondata
}

#[test]
fn t_parse_invalid_codice_checkchar() {
    assert_eq!(
        format!( "{}", CodiceFiscale::parse(TEST_CF_ERR_CHECKCHAR).err().unwrap() ),
        "invalid-checkchar"
    );
}

#[test]
fn t_check_ok() {
    assert_eq!(CodiceFiscale::check(TEST_CF_OK), true);
}