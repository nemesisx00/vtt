use ::chrono::{DateTime, Utc};

pub fn parseDateTime(timestamp: Option<&String>) -> Option<DateTime<Utc>>
{
	return match timestamp
	{
		None => None,
		Some(ts) => match ts.parse::<i64>()
		{
			Err(_) => None,
			Ok(number) => DateTime::from_timestamp(number, 0),
		},
	};
}
