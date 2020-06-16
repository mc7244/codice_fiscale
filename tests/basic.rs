#![cfg(test)]
extern crate codice_fiscale;
use codice_fiscale::*;

const TEST_CF_OK: &str = "BLTMHL77S04E889G";
const TEST_CF_ERR_CHECKCHAR: &str = "BLTMHL77S04E889Y";
const TEST_MUNICIPALITY: &str = "Maniago";

fn make_new_test_persondata() -> PersonData {
    let store = belfiore::Belfiore::init();
    PersonData {
        name: "Michele".to_string(),
        surname: "Beltrame".to_string(),
        birthdate: "1977-11-04".to_string(),
        gender: Gender::M,
        place_of_birth: store.get_info(TEST_MUNICIPALITY).unwrap().clone(),
    }
}

fn make_parse_test_persondata() -> PersonData {
    let store = belfiore::Belfiore::init();
    PersonData {
        name: "MHL".to_string(),
        surname: "BLT".to_string(),
        birthdate: "1977-11-04".to_string(),
        gender: Gender::M,
        place_of_birth: store.get_info(TEST_MUNICIPALITY).unwrap().clone(),
    }
}

#[test]
fn t_new() {
    let persondata = make_new_test_persondata();
    assert_eq!(
        CodiceFiscale::new(&persondata).unwrap().get_codice(),
        TEST_CF_OK
    );
}

#[test]
fn t_new_err_belfiore() {
    let store = belfiore::Belfiore::init();
    assert!(store.lookup_belfiore("EX").is_none());
}

#[test]
fn t_new_err_birthdate() {
    let mut persondata = make_new_test_persondata();
    persondata.birthdate = "1977-04-32".to_string();
    assert_eq!(
        format!("{}", CodiceFiscale::new(&persondata).err().unwrap()),
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
    assert_eq!(cf.get_person_data().place_of_birth.belfiore_code, "E889");
}

#[test]
fn t_parse_ok() {
    let cf = CodiceFiscale::parse(TEST_CF_OK).unwrap();
    let persondata = make_parse_test_persondata();
    assert_eq!(cf.get_person_data(), &persondata);
    // TODO: check whole persondata
}

#[test]
fn t_parse_invalid_codice_checkchar() {
    assert_eq!(
        format!(
            "{}",
            CodiceFiscale::parse(TEST_CF_ERR_CHECKCHAR).err().unwrap()
        ),
        "invalid-checkchar"
    );
}

#[test]
fn t_check_ok() {
    assert_eq!(CodiceFiscale::check(TEST_CF_OK).is_ok(), true);
}

#[test]
fn t_check_name_validity() {
    let persondata = make_new_test_persondata();
    assert!(
        CodiceFiscale::new(&persondata).unwrap().is_name_valid(&persondata.name)
    );
}

#[test]
fn t_check_surname_validity() {
    let persondata = make_new_test_persondata();
    assert!(
        CodiceFiscale::new(&persondata).unwrap().is_surname_valid(&persondata.surname)
    );
}
