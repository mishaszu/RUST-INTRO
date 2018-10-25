const NTHS: [&'static str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    "eleventh", "twelfth",
];

pub fn get_nth(n: usize) -> &'static str {
    NTHS[n]
}
