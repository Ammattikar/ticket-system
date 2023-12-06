use crate::{
	prelude::*,
	ticket::Ticket
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
	#[serde(default)]
	id: SeatId,
	/// The train this seat is located on.
	train_id: TrainId,
	name: String,
	/// The price to book one seat in this class, excluding fees.
	price: f32,  // not safe, but this is not a real application
}

/// List all trains in the database.
#[get("/list")]
pub fn list_trains(db: &State<Database>) -> RocketJson<Vec<Train>> {
	RocketJson(db.read_all(TABLE_TRAINS).expect("failed to read trains list"))
}

/// Get a specific train by ID.
#[get("/get/<id>")]
pub fn get_train(id: u64, db: &State<Database>) -> Option<RocketJson<Train>> {
	let id = TrainId(id); // not safe, but probably fine
	db.read_item(id, TABLE_TRAINS).expect("failed to read train").map(RocketJson)
}

/// Create a train with an automatically generated ID.
#[post("/create", data = "<train>")]
pub fn create_train(mut train: RocketJson<Train>, db: &State<Database>) -> RocketJson<u64> {
	let id = db.get_monotonic_id();
	train.id = TrainId(id);

	db.write_item(id, &train.0, b"trains").expect("failed to write train");
	RocketJson(id)
}

/// Create a seat with an automatically generated ID.
#[post("/create_seat", data = "<seat>")]
pub fn create_seat(mut seat: RocketJson<Seat>, db: &State<Database>) -> RocketJson<SeatId> {
	let id = db.get_monotonic_id();
	seat.id = SeatId(id);

	db.write_paired_item(seat.train_id.0, id, &seat.0, TABLE_BIKEY_SEATS).expect("failed to create seat");

	RocketJson(seat.id)
}

/// List all seats associated with a specific train.
#[get("/list_seats/<train_id>")]
pub fn list_seats(train_id: u64, db: &State<Database>) -> RocketJson<Vec<Seat>> {
	let train_id = TrainId(train_id);
	let mut seats = Vec::new();
	let known_seats: BTreeMap<SeatId, Seat> = db.scan_items_by_prefix(train_id, TABLE_BIKEY_SEATS).expect("unable to read seat information");
	for (_id, seat) in known_seats {
		seats.push(seat);
	}

	RocketJson(seats)
}

/// List all tickets associated with a specific timeslot and train.
#[get("/tickets/<train_id>/<schedule_id>")]
pub fn list_tickets(train_id: u64, schedule_id: u64, db: &State<Database>) -> RocketJson<Vec<Ticket>> {
	let mut ret = Vec::new();
	let tickets: BTreeMap<TicketId, Ticket> = db.scan_items_by_prefix(schedule_id, TABLE_BIKEY_TICKETS_BY_DEPARTURE).expect("unable to read ticket information");
	for (_id, ticket) in tickets {
		if ticket.train.0 != train_id {
			continue;
		}
		ret.push(ticket);
	}

	RocketJson(ret)
}

/// List all seats on a train that are unoccupied (no ticket) for a specific timeslot.
#[get("/available_seats/<train_id>/<schedule_id>")]
pub fn available_seats(train_id: u64, schedule_id: u64, db: &State<Database>) -> RocketJson<Vec<Seat>> {
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

	let mut seats = Vec::new();
	for (_id, seat) in known_seats {
		seats.push(seat);
	}

	RocketJson(seats)
}
