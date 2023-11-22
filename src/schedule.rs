use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct ScheduledDeparture {
	id: ScheduledDepartureId,
	train: TrainId,
	time: std::time::SystemTime,
}
