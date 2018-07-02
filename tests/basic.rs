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
        CodiceFiscale::new(&persondata).unwrap().codice(),
        "BLTMHL77S04E889G".to_string()
    );
}

#[test]
fn t_new_err_comune() {
    let mut persondata = init_sample1();
    persondata.comune = "EX".to_string();
    assert_eq!(
        format!("{}",CodiceFiscale::new(&persondata).err().unwrap()),
        "invalid-belfiore-code"
    );
}

#[test]
fn t_new_err_birthdate() {
    let mut persondata = init_sample1();
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
        let pdata = init_sample1();
        cf = CodiceFiscale::new(&pdata).unwrap();
    }
    assert_eq!(cf.persondata().comune, "E889");
}

#[test]
fn t_parse_ok() {
    let cf = CodiceFiscale::parse("BLTMHL77S04E889G");
    // TODO: check whole persondata
    assert_eq!(cf.unwrap().codice(), "BLTMHL77S04E889G");
}

#[test]
fn t_check_ok() {
    assert_eq!(CodiceFiscale::check("BLTMHL77S04E889G"), true);
}