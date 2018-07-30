extern crate conway;
extern crate ws;

use std::fmt;
use std::io::{stderr, Write};
use std::sync::{Arc, Mutex};

use conway::{Cell, Game, Grid, GridConfig, View};

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
            game: Arc::new(Mutex::new(Server::new_game())),
        }
    }

    fn new_game() -> Game {
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

    fn alert<T: fmt::Display + Into<ws::Message>>(&self, msg: T) -> ws::Result<()> {
        write!(stderr(), "{}", msg)?;
        self.out.send(msg)
    }
}

impl ws::Handler for Server {
    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        let mut game = self.game.lock().unwrap();

        let mut args = msg.as_text()?.trim().splitn(2, ' ');
        match args.next() {
            Some("tick") => {
                game.tick();
                self.out.send(game.draw())
            }
            // TODO
            Some("toggle-playback") => Ok(()),
            Some("scroll") => {
                let Cell(dx, dy): Cell = match args.next().unwrap_or_default().parse::<Cell>() {
                    Ok(delta) => delta,
                    Err(err) => return self.alert(format!("WARNING: {}", err)),
                };
                game.scroll(dx, dy);
                Ok(())
            }
            Some(arg) => self.alert(format!(
                "WARNING: message contained unexpected command '{}'",
                arg
            )),
            None => self.alert("WARNING: empty message received"),
        }
    }

    fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()> {
        let mutex = Arc::get_mut(&mut self.game).unwrap();
        *mutex.get_mut().unwrap() = Server::new_game();
        Ok(())
    }
}

fn main() {
    ws::listen(WS_ADDR, Server::new).unwrap();
}
