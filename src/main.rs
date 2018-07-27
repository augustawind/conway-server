#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate serde_derive;

extern crate conway;
extern crate rocket;
extern crate rocket_contrib;

use std::sync::{Arc, Mutex};

use rocket::State;
use rocket_contrib::Template;

use conway::{config::GridConfig, grid::Grid, Game};

static DEFAULT_PATTERN: &str = r#"
.......
...x...
....x..
..xxx..
.......
"#;

#[derive(Serialize)]
struct Context {
    pub game_output: String,
}

#[get("/", format = "text/html")]
fn route_index(state: State<Arc<Mutex<Game>>>) -> Template {
    let game = state.lock().expect("unlocking mutex failed");
    let context = Context {
        game_output: game.draw(),
    };
    Template::render("index", &context)
}

fn main() {
    // FIXME: make this load at compile time
    let grid = Grid::from_config(GridConfig {
        pattern: DEFAULT_PATTERN.to_string(),
        ..Default::default()
    }).expect("Something went wrong with loading grid");
    let game = Game::new(grid, Default::default());

    rocket::ignite()
        .mount("/", routes![route_index])
        .attach(Template::fairing())
        .manage(Arc::new(Mutex::new(game)))
        .launch();
}
