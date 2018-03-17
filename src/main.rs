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

#[derive(Serialize, Deserialize)]
struct ReportDocument {
    created_at: i64,
    project: String,
    report: Vec<coala_types::Report>
}

impl ReportDocument {
    pub fn to_bson(&self) -> bson::ordered::OrderedDocument {
        doc! {
            "created_at" => self.created_at.to_owned(),
            "project" => self.project.to_owned(),
            "report" => bson::to_bson(&self.report).expect("Failed to encode report"),
        }
    }

    fn from_doc(doc: &bson::Document) -> Self {
        bson::from_bson(bson::Bson::Document(doc.clone())).unwrap()
    }
}

#[post("/<project>", format = "application/json", data = "<coala>")]
fn report(state: State<AppState>, project: String, coala: Json<coala_types::Coala>) -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time retrieval");

    let client = state.client_pool.pop();
    let reports = client.get_collection("coala-collector", "report");

    let document = ReportDocument {
        created_at: now.as_secs() as i64,
        project: project,
        report: coala.results.cli.clone()
    };

    reports.insert(&document.to_bson(), None).expect("Failed to insert document");
    format!("Report, content: \n{:?}", coala)
}

#[get("/<project>")]
fn get_reports(state: State<AppState>, project: String) -> Json<Vec<ReportDocument>> {
    let client = state.client_pool.pop();
    let collection = client.get_collection("coala-collector", "report");
    let query_project = doc! {
        "project" => project,
    };

    let cursor = collection.find(&query_project, None).expect("Failed to find documents");
    let reports = cursor.into_iter().map(|d| ReportDocument::from_doc(&d.expect("Result"))).collect();
    Json(reports)
}

struct AppState {
    client_pool: Arc<ClientPool>
}

fn main() {
    let uri = Uri::new("mongodb://localhost:27017/").unwrap();
    let pool = Arc::new(ClientPool::new(uri.clone(), None));
    let state = AppState { client_pool: pool };
    rocket::ignite().manage(state).mount("/report", routes![report, get_reports]).launch();
}
