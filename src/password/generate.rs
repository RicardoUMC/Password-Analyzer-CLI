use rand::seq::{IndexedRandom, SliceRandom};

pub fn generate_password(length: usize) -> String {
    let mut rng = rand::rng();

    let lowercase: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let uppercase: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let digits: Vec<char> = "0123456789".chars().collect();
    let specials: Vec<char> = "!@#$%^&*()-_+=.".chars().collect();

    let mut all_chars = lowercase.clone();
    all_chars.extend(&uppercase);
    all_chars.extend(&digits);
    all_chars.extend(&specials);

    let mut password = vec![
        *lowercase.choose(&mut rng).unwrap(),
        *uppercase.choose(&mut rng).unwrap(),
        *digits.choose(&mut rng).unwrap(),
        *specials.choose(&mut rng).unwrap(),
    ];

    for _ in 0..(length - 4) {
        password.push(*all_chars.choose(&mut rng).unwrap());
    }

    password.shuffle(&mut rng);
    password.into_iter().collect()
}
