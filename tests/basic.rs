#![cfg(test)]
extern crate codice_fiscale;
use codice_fiscale::*;

fn init_sample1() -> PersonData {
    PersonData {
        name: "Michele".to_string(),
        surname: "Beltrame".to_string(),
        birthdate: "1977-11-04".to_string(),
        gender: Gender::M,
        comune: "E889".to_string(),
    }
}

#[test]
fn t_new() {
    let persondata = init_sample1();
    assert_eq!(
        CodiceFiscale::new(&persondata).unwrap(),
        CodiceFiscale { persondata: persondata, codice : "BLTMHL77S04E889G".to_string()
    });
}

#[test]
fn t_new_err_comune() {
    let mut persondata = init_sample1();
    persondata.comune = "EX".to_string();
    assert_eq!(CodiceFiscale::new(&persondata).err().unwrap().description(), "invalid-comune");
}

#[test]
fn t_comune_ok() {
    let cf = CodiceFiscale::new(&init_sample1()).unwrap();
    assert_eq!(cf.persondata.comune, "E889");
}