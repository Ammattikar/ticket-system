use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct ScheduledDeparture {
	#[serde(default)]
	id: ScheduledDepartureId,
	train: TrainId,
	time: u64,	// seconds since UNIX epoch
}

#[post("/create", data = "<schedule>")]
pub fn create_schedule(mut schedule: RocketJson<ScheduledDeparture>, db: &State<Database>) {
	let id = db.get_monotonic_id();
	schedule.id = ScheduledDepartureId(id);

	db.write_item(schedule.id, &schedule.0, TABLE_SCHEDULES).expect("failed to write schedule");
	db.write_paired_item(schedule.train, id, &schedule.0, TABLE_BIKEY_DEPARTURES_BY_TRAIN).expect("failed to write schedule train index");
	db.write_paired_item(schedule.time, id, &schedule.0, TABLE_BIKEY_DEPARTURES_BY_TIME).expect("failed to write schedule time index");
}

#[get("/list")]
pub fn list_schedules(db: &State<Database>) -> RocketJson<Vec<ScheduledDeparture>> {
	RocketJson(db.read_all(TABLE_SCHEDULES).expect("failed to read schedules list"))
}
