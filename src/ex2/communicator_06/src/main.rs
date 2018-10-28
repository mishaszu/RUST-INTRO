extern crate communicator_06;

use communicator_06::client;
use communicator_06::network::server::connect;

enum TrafficLights {
    Red,
    Yellow,
    Green,
}

use self::TrafficLights::*;

fn use_1() {
    let l1 = Red;
    let l2 = Yellow;
    let l3 = Green;
}

fn main() {
    println!("Running modules");
    communicator_06::client::connect();
    use_1();
}
