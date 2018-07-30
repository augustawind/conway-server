extern crate conway;
extern crate ws;

use std::fmt;
use std::io::{stderr, Write};
use std::sync::{Arc, Mutex};

use conway::{Cell, Game, Grid, GridConfig, View};

static WS_ADDR: &str = "localhost:3012";

static CHAR_ALIVE: char = '■';
static CHAR_DEAD: char = '□';
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
    paused: bool,
}

impl Server {
    fn new(out: ws::Sender) -> Self {
        Server {
            out,
            game: Arc::new(Mutex::new(Server::new_game())),
            paused: false,
        }
    }

    fn new_game() -> Game {
        let char_alive = CHAR_ALIVE.to_string();
        let char_dead = CHAR_DEAD.to_string();
        Game::new(
            Grid::from_config(GridConfig {
                pattern: DEFAULT_PATTERN
                    .replace('x', char_alive.as_str())
                    .replace('.', char_dead.as_str())
                    .to_string(),
                char_alive: CHAR_ALIVE,
                char_dead: CHAR_DEAD,
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
            Some("ping") => {
                if self.paused {
                    return Ok(());
                }
                game.tick();
                self.out.send(game.draw())
            }
            Some("step") => {
                if !self.paused {
                    return Ok(());
                }
                game.tick();
                self.out.send(game.draw())
            }
            Some("toggle-playback") => {
                self.paused = !self.paused;
                if !self.paused {
                    game.tick();
                    return self.out.send(game.draw());
                }
                Ok(())
            }
            Some("scroll") => {
                let Cell(dx, dy): Cell = match args.next().unwrap_or_default().parse::<Cell>() {
                    Ok(delta) => delta,
                    Err(err) => return self.alert(format!("WARNING: {}", err)),
                };
                game.scroll(dx, dy);
                self.out.send(game.draw())
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
