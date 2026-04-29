use rand::RngExt;

pub fn generate_random_number(min: i32, max: i32) -> i32 {
    let mut rng = rand::rng();
    let value = rng.random_range(min..max);
    value
}

pub fn get_random_key<'a>(keys: &'a Vec<String>, not_available: Option<&String>) -> &'a String {
    let filtered_keys: Vec<&String> = if let Some(exclude) = not_available {
        keys.iter().filter(|k| *k != exclude).collect()
    } else {
        keys.iter().collect()
    };

    let mut rng = rand::rng();
    let index = rng.random_range(0..filtered_keys.len());

    filtered_keys[index]
}

