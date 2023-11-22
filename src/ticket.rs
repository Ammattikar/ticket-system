use crate::prelude::*;

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

#[post("/create", data = "<ticket>")]
pub fn create_ticket(mut ticket: RocketJson<Ticket>, db: &State<Database>) {
	ticket.id = TicketId(db.get_monotonic_id());
	db.write_item(ticket.id, &ticket.0, TABLE_TICKETS).expect("failed to write ticket");

}
