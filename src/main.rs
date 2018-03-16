#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use rocket_contrib::Json;

#[macro_use]
mod serde_util;
mod coala_types;

#[post("/<project>", format = "application/json", data = "<coala>")]
fn report(project: String, coala: Json<coala_types::Coala>) -> String {
    format!("Report, project {} content: \n{:?}", project, coala)
}

fn main() {
    rocket::ignite().mount("/report", routes![report]).launch();
}
