use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct ScheduledDeparture {
	id: ScheduledDepartureId,
	train: TrainId,
	time: std::time::SystemTime,
}

#[post("/create", data = "<schedule>")]
pub fn create_schedule(mut schedule: RocketJson<ScheduledDeparture>, db: &State<Database>) {
	let id = db.get_monotonic_id();
	schedule.id = ScheduledDepartureId(id);

	db.write_item(schedule.id, &schedule.0, TABLE_SCHEDULES);
}

#[get("/list")]
pub fn list_schedules(db: &State<Database>) -> RocketJson<Vec<ScheduledDeparture>> {
	RocketJson(db.read_all(TABLE_SCHEDULES).expect("failed to read schedules list"))
}
