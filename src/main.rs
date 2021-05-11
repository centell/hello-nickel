#[macro_use] extern crate nickel;

use nickel::{Nickel, HttpRouter};

fn main() {
    let mut server = Nickel::new();
    server.get("**", middleware!("Hello, World"));
    let listening = server.listen("127.0.0.1:6767").expect("Failed to launch server");
    println!("Listening on: {:?}", listening.socket());
}
