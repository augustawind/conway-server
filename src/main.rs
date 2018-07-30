#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

extern crate conway;
extern crate rocket;
extern crate rocket_contrib;

use std::path::{Path, PathBuf};

use rocket::response::NamedFile;
use rocket_contrib::Template;

#[derive(Serialize)]
struct Context {
    title: &'static str,
    default_grid: &'static str,
}

lazy_static! {
    static ref PATH_STATIC: &'static Path = Path::new("static/");
    static ref CONTEXT: &'static Context = &Context {
        title: "Conway's Game of Life",
        default_grid: r#"
..........
...x......
....x.....
..xxx.....
..........
..........
..........
..........
..........
        "#,
    };
}

#[get("/", format = "text/html")]
fn route_index() -> Template {
    Template::render("index", *CONTEXT)
}

#[get("/static/<file..>")]
fn route_static(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(PATH_STATIC.join(file)).ok()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![route_index, route_static])
        .attach(Template::fairing())
        .launch();
}
