#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use(bson, doc)]
extern crate bson;
extern crate mongo_driver;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use mongo_driver::client::{ClientPool,Uri};
use rocket_contrib::Json;
use rocket::State;

#[macro_use]
mod serde_util;
mod coala_types;

#[post("/<project>", format = "application/json", data = "<coala>")]
fn report(state: State<AppState>, project: String, coala: Json<coala_types::Coala>) -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time retrieval");

    let client = state.client_pool.pop();
    let reports = client.take_collection("coala-collector", "report");
    let report = bson::to_bson(&coala.results.cli).expect("Failed to encode report");

    let document = doc! {
        "created_at" => now.as_secs(),
        "project" => project,
        "report" => report,
    };
    reports.insert(&document, None).expect("Failed to insert document");
    format!("Report, content: \n{:?}", coala)
}

struct AppState {
    client_pool: Arc<ClientPool>
}

fn main() {
    let uri = Uri::new("mongodb://localhost:27017/").unwrap();
    let pool = Arc::new(ClientPool::new(uri.clone(), None));
    let state = AppState { client_pool: pool };
    rocket::ignite().manage(state).mount("/report", routes![report]).launch();
}
