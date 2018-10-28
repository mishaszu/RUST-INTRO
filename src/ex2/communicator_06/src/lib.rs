pub mod client;
pub mod network;

#[cfg(test)]
mod test {
    use super::client;

    #[test]
    fn it_works() {
        client::connect();
    }
}
