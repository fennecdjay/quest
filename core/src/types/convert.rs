use crate::{Object, Result, Args};
use std::any::Any;

pub trait Convertible : Any + Sized + Clone {
	const CONVERT_FUNC: &'static str;
}

impl Object {
	pub fn downcast_convert<T: Convertible>(&self) -> Result<Self> {
		self.call_attr(T::CONVERT_FUNC, Args::default())
	}

	pub fn downcast_call<T: Convertible>(&self) -> Result<T> {
		self.downcast_convert::<T>().and_then(|o| o.try_downcast_clone())
	}
}