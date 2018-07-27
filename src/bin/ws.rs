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

struct Server {
    out: ws::Sender,
    game: Arc<Mutex<Game>>,
}

impl Server {
    fn new(out: ws::Sender, game: Game) -> Self {
        Server {
            out,
            game: Arc::new(Mutex::new(game)),
        }
    }
}

impl ws::Handler for Server {
    fn on_message(&mut self, _: ws::Message) -> ws::Result<()> {
        let mut game = self.game.lock().unwrap();
        game.tick();
        self.out.send(game.draw())
    }

    fn on_close(&mut self, _: ws::CloseCode, _: &str) {
        self.out.shutdown().unwrap();
    }
}

fn main() {
    ws::listen(WS_ADDR, |out| {
        Server::new(
            out,
            Game::new(
                Grid::from_config(GridConfig {
                    pattern: DEFAULT_PATTERN.to_string(),
                    ..Default::default()
                }).unwrap(),
                Default::default(),
            ),
        )
    }).unwrap();
}
