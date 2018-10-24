#[cfg(test)]

mod tests {
    extern crate customCrate;
    #[test]
    fn english_greeting_correct() {
        assert_eq!("hello", greetings::english::hello())
    }

    #[test]
    #[should_panic]
    fn english_greeting_correct2() {
        assert_eq!("helloa", greetings::english::hello())
    }
}
