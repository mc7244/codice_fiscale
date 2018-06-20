use std::collections::HashMap;

lazy_static! {
    pub static ref CHECKCHARS: HashMap<char, (usize, usize)> = {
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
