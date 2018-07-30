extern crate conway_server;

use conway_server::pubsub;

const ADDR: &str = "localhost:3012";

fn main() {
    pubsub::listen(ADDR).unwrap();
}
