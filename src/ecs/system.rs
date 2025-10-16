pub trait SystemTrait {
	fn tick(&mut self, delta_t: u64);
}
