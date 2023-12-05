use crate::prelude::*;

// Just a bunch of newtypes to avoid having a bunch of unmarked u64s lying around.

// These types are all the same, might as well save some typing.
macro_rules! id_type {
	($name:ident) => {
		#[repr(transparent)]
		#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default, PartialEq, Eq)]
		#[serde(transparent)]
		pub struct $name(pub u64);

		impl From<$name> for u64 {
			fn from(val: $name) -> Self {
				val.0
			}
		}

		impl From<u64> for $name {
			fn from(val: u64) -> Self {
				$name(val)
			}
		}

		impl Ord for $name {
			fn cmp(&self, other: &Self) -> std::cmp::Ordering {
				self.0.cmp(&other.0)
			}
		}

		impl PartialOrd for $name {
			fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
				Some(self.0.cmp(&other.0))
			}
		}
	};
}

id_type!(SeatId);
id_type!(TrainId);
id_type!(TicketId);
id_type!(ScheduledDepartureId);
