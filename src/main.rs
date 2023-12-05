#[macro_use]
extern crate rocket;
#[macro_use]
extern crate log;

pub mod prelude {
	// pub use diesel::Queryable;
	pub use serde_derive::{Deserialize, Serialize};
	pub use crate::{
		constants::*,
		database::Database,
		id_types::*
	};
	pub use rocket::serde::json::Json as RocketJson;
	pub use rocket::State;
}

// mod auth;
mod constants;
mod database;
mod debug;
mod id_types;
mod schedule;
mod ticket;
mod train;

#[launch]
fn rocket() -> _ {
	let db = database::Database::new();
	db.init().expect("database failed to initialize");

	rocket::build()
		.manage(db)
		.mount("/train", routes![
			train::get_train,
			train::create_train,
			//train::delete_train,
			train::list_trains,
			train::available_seats
		])
		.mount("/ticket", routes![
			ticket::create_ticket,
			//ticket::delete_ticket,
		])
		.mount("/schedule", routes![
			schedule::create_schedule,
			schedule::list_schedules,
		])
		.mount("/", routes![
			//todo: static asset root
		])
}
