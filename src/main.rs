#[macro_use]
extern crate rocket;

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

mod auth;
mod constants;
mod schedule;
mod ticket;
mod train;
mod id_types;
mod database;

#[launch]
fn rocket() -> _ {
	let db = database::Database::new();
	db.init().expect("database failed to initialize");

	rocket::build()
		.manage(db)
		.mount("/auth", routes![
			auth::login,
			auth::logout
		])
		.mount("/train", routes![
			train::get_train,
			train::create_train,
			train::list_trains,
			train::available_seats
		])
		.mount("/ticket", routes![
			ticket::create_ticket
		])
		.mount("/schedule", routes![])
}
