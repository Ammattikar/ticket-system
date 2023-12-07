#[macro_use]
extern crate rocket;

pub mod prelude {
	pub use serde_derive::{Deserialize, Serialize};
	pub use crate::{
		constants::*,
		database::Database,
		id_types::*
	};
	pub use rocket::serde::json::Json as RocketJson;
	pub use rocket::State;
}

mod constants;
mod database;
mod id_types;
mod schedule;
mod ticket;
mod train;

use rocket::{
	fairing::AdHoc,
	http::Header
};

#[launch]
fn rocket() -> _ {
	let db = database::Database::new();
	db.init().expect("database failed to initialize");

	rocket::build()
		.attach(AdHoc::on_response("Fix CORS", |_req, resp| Box::pin(async move {
			resp.set_header(Header::new("Access-Control-Allow-Origin", "*"));
			resp.set_header(Header::new("Access-Control-Allow-Methods", "GET, POST"));
			resp.set_header(Header::new("Access-Control-Allow-Headers", "*"));
			resp.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
		})))
		.manage(db)
		.mount("/train", routes![
			train::get_train,
			train::create_train,
			//train::delete_train,
			train::list_trains,
			train::create_seat,
			train::edit_seat,
			train::list_seats,
			train::available_seats,
			train::list_tickets,
		])
		.mount("/ticket", routes![
			ticket::create_ticket,
			ticket::delete_ticket,
			ticket::edit_ticket,
		])
		.mount("/schedule", routes![
			schedule::create_schedule,
			schedule::edit_schedule,
			schedule::list_schedules,
			schedule::list_after_time,
		])
		.mount("/", routes![
			all_options
		])
}

#[options("/<_..>")]
fn all_options() {
	/* Intentionally left empty */
}
