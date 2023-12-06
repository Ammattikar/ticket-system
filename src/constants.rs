/// List of train objects by ID.
pub const TABLE_TRAINS: &[u8] = b"trains";

/// List of ticket objects by ID.
pub const TABLE_TICKETS: &[u8] = b"tickets";

/// List of schedule objects by ID.
pub const TABLE_SCHEDULES: &[u8] = b"schedules";

/// List of schedules partitioned by train ID.
pub const TABLE_BIKEY_DEPARTURES_BY_TRAIN: &[u8] = b"departures_by_train";

/// List of schedules partitioned by time.
pub const TABLE_BIKEY_DEPARTURES_BY_TIME: &[u8] = b"departures_by_time";

/// List of seats partitioned by train ID.
pub const TABLE_BIKEY_SEATS: &[u8] = b"seats";

/// List of tickets partitioned by schedule ID.
pub const TABLE_BIKEY_TICKETS_BY_DEPARTURE: &[u8] = b"tickets_by_departure";
