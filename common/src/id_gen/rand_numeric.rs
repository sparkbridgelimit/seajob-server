use rand::{thread_rng, Rng};
pub fn rand_numeric(length: usize) -> String {
    let mut rng = thread_rng();
    (0..length)
        .map(|_| rng.gen_range(0..10))
        .map(|num| char::from_digit(num, 10).unwrap())
        .collect()
}