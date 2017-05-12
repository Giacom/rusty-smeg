use services::service::Service;

#[derive(Default)]
pub struct Time {
	pub ticks: u64,
	pub delta_time: f32
}

impl Time {
	pub fn new() -> Time {
		Time::default()
	}
}

impl Service for Time {

}