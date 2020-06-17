/// This struct represents a municipality
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Municipality {
    pub name: String,
    pub province: String,
    pub belfiore_code: String,
}

/// The database, you can query it using the following functions
pub struct Belfiore {
    store: Vec<Municipality>,
}

impl Belfiore {
    /// Initialize the struct using belfiore.txt
    pub fn init() -> Self {
        let db: Vec<Municipality> = include_str!("../belfiore.txt")
            .split('\n')
            .map(|x| x.split(',').collect::<Vec<&str>>())
            .map(|x| Municipality {
                name: x[2].to_owned(),
                province: x[1].to_owned(),
                belfiore_code: x[0].to_owned(),
            })
            .collect();
        Self { store: db }
    }
    /// Obtain info for a municipality (name, province and Belfiore code)
    pub fn get_info(&self, municipality_name: &str) -> Option<&Municipality> {
        self.store
            .iter()
            .find(|x| x.name == municipality_name.to_uppercase())
    }
    /// Obtain info for a Belfiore code
    pub fn lookup_belfiore(&self, belfiore: &str) -> Option<&Municipality> {
        self.store
            .iter()
            .find(|x| x.belfiore_code == belfiore.to_uppercase())
    }
}
