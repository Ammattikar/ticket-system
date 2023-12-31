use crate::prelude::*;
use rocket::http::Status;

#[derive(Debug, Serialize, Deserialize)]
pub struct Ticket {
	/// An auto-incremented ID for this ticket.
	#[serde(default)]
	pub id: TicketId,
	/// The seat this ticket is booking.
	pub seat: SeatId,
	/// The train this ticket is associated with.
	pub train: TrainId,
	/// The timeslot that this ticket is for.
	pub departure: ScheduledDepartureId,
	/// The name of the customer associated with this ticket.
	pub customer_name: String,
	/// The price that the customer paid for the ticket, including fees if applicable.
	pub price_paid: f32,
}

/// Create a ticket with an automatically generated ID.
#[post("/create", data = "<ticket>")]
pub fn create_ticket(mut ticket: RocketJson<Ticket>, db: &State<Database>) -> RocketJson<u64> {
	ticket.id = TicketId(db.get_monotonic_id());
	db.write_item(ticket.id, &ticket.0, TABLE_TICKETS).expect("failed to write ticket");
	db.write_paired_item(ticket.departure.0, ticket.id.0, &ticket.0, TABLE_BIKEY_TICKETS_BY_DEPARTURE).expect("failed to write ticket index");
	RocketJson(ticket.id.0)
}

#[post("/edit/<id>", data = "<ticket>")]
pub fn edit_ticket(id: u64, mut ticket: RocketJson<Ticket>, db: &State<Database>) -> Status {
	ticket.id = TicketId(id);
	if db.contains(ticket.id, TABLE_TICKETS).expect("failed to read ticket") {
		db.write_item(ticket.id, &ticket.0, TABLE_TICKETS).expect("failed to write ticket");
		db.write_paired_item(ticket.departure.0, ticket.id.0, &ticket.0, TABLE_BIKEY_TICKETS_BY_DEPARTURE).expect("failed to write ticket index");

		Status::Ok
	} else {
		Status::NotFound
	}
}

/// Delete a ticket by ID, removing it from indexes.
#[post("/delete/<ticket>")]
pub fn delete_ticket(ticket: u64, db: &State<Database>) {
	let id = TicketId(ticket);
	if let Some(ticket) = db.delete_item::<_, Ticket>(id, TABLE_TICKETS).expect("failed to delete ticket") {
		db.delete_paired_item::<_, _, Ticket>(ticket.departure.0, id.0, TABLE_BIKEY_TICKETS_BY_DEPARTURE).expect("failed to delete ticket from index");
	}
}
