extern crate codice_fiscale;
use codice_fiscale::*;

#[test]
fn calculate_works() {
    let persondata = PersonData {
        name: "Michele".to_string(),
        surname: "Beltrame".to_string(),
        birthdate: "1977-11-04".to_string(),
        sex: Sex::M,
        comune: "E889".to_string(),
    };
    assert_eq!(calculate(&persondata).unwrap(), "CIAO");
}