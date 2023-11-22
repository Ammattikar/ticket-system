use crate::{
	prelude::*,
	ticket::Ticket,
	schedule::ScheduledDeparture,
};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Train {
	/// An auto-incremented ID with no inherent meaning.
	#[serde(default)]
	id: TrainId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Seat {
	/// An auto-incremented ID with no inherent meaning.
	id: SeatId,
	/// The train this seat is located on.
	train_id: TrainId,
	/// The pricing class this seat is a member of.
	seat_class: SeatClassId,
}

#[derive(Debug)]
struct SeatClass {
	/// An auto-incremented ID with no inherent meaning.
	id: SeatClassId,
	/// The user-visible name for this class of seats, such as 'First Class'.
	name: String,
	/// The price to book one seat in this class, excluding fees.
	price: f32,  // not safe, but this is not a real application
}

#[get("/list")]
pub fn list_trains(db: &State<Database>) -> RocketJson<Vec<TrainId>> {
	RocketJson(db.list_item(TABLE_TRAINS).expect("failed to read train list"))
}

#[get("/get/<id>")]
pub fn get_train(id: u64, db: &State<Database>) -> Option<RocketJson<Train>> {
	let id = TrainId(id); // not safe, but probably fine
	db.read_item(id, TABLE_TRAINS).expect("failed to read train").map(RocketJson)
}

#[post("/create", data = "<train>")]
pub fn create_train(mut train: RocketJson<Train>, db: &State<Database>) {
	let id = db.get_monotonic_id();
	train.id = TrainId(id);

	db.write_item(id, &train.0, b"trains").expect("failed to write train");
}

#[get("/available_seats/<train_id>/<schedule_id>")]
pub fn available_seats(train_id: u64, schedule_id: u64, db: &State<Database>) -> RocketJson<Vec<SeatId>> {
	let train_id = TrainId(train_id);
	let schedule_id = ScheduledDepartureId(schedule_id);
	/*
		For seat in train, check if seat is allocated to ticket -- if no, return.
	 */

	let mut known_seats: BTreeMap<SeatId, Seat> = db.scan_items_by_prefix(train_id, TABLE_BIKEY_SEATS).expect("unable to read seat information");
	let tickets: BTreeMap<TicketId, Ticket> = db.scan_items_by_prefix(schedule_id, TABLE_BIKEY_TICKETS_BY_DEPARTURE).expect("unable to read ticket information");
	for (_id, ticket) in tickets {
		if ticket.train != train_id {
			continue;
		}
		known_seats.remove(&ticket.seat);
	}

	return RocketJson(known_seats.keys().cloned().collect())
}

pub fn list_tickets_for_train(train_id: u64, db: &State<Database>) -> RocketJson<Vec<Ticket>> {
	let train_id = TrainId(train_id);

	todo!()
}
