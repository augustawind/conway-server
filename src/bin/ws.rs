extern crate conway;
extern crate ws;

use std::sync::{Arc, Mutex};

use conway::{Game, Grid, GridConfig, View};

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
    fn new(out: ws::Sender) -> Self {
        Server {
            out,
            game: Arc::new(Mutex::new(Server::default_game())),
        }
    }

    fn default_game() -> Game {
        Game::new(
            Grid::from_config(GridConfig {
                pattern: DEFAULT_PATTERN.to_string(),
                view: View::Fixed,
                width: 50,
                height: 50,
                ..Default::default()
            }).unwrap(),
            Default::default(),
        )
    }
}

impl ws::Handler for Server {
    fn on_message(&mut self, _: ws::Message) -> ws::Result<()> {
        let mut game = self.game.lock().unwrap();
        game.tick();
        self.out.send(game.draw())
    }

    fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()> {
        let mutex = Arc::get_mut(&mut self.game).unwrap();
        *mutex.get_mut().unwrap() = Server::default_game();
        Ok(())
    }
}

fn main() {
    ws::listen(WS_ADDR, Server::new).unwrap();
}
