extern crate conway;
extern crate ws;

use std::sync::{Arc, Mutex};

use conway::{Game, Grid, GridConfig};

static WS_ADDR: &str = "localhost:3012";

static DEFAULT_PATTERN: &str = r#"
.......
...x...
....x..
..xxx..
.......
"#;

// struct Server {
//     out: ws::Sender,
//     game: Game,
// }

// impl Handler for Server {
//     fn new
// }

fn main() {
    ws::listen(WS_ADDR, |out| {
        let game = Arc::new(Mutex::new(Game::new(
            Grid::from_config(GridConfig {
                pattern: DEFAULT_PATTERN.to_string(),
                ..Default::default()
            }).unwrap(),
            Default::default(),
        )));
        move |_| {
            println!("Message received!");
            let mut guard = game.lock().unwrap();
            for output in guard.iter() {
                println!("Sending output... {}", output);
                out.send(output)?;
            }
            Ok(())
        }
    }).unwrap();
}
