#![deny(unused_must_use)]
#![allow(unused)]
#![allow(deprecated)]

macro_rules! parse_error {
	(context=$context:expr, $type:ident $($tt:tt)*) => {
		$crate::Error::new($context, $crate::ErrorType::$type$($tt)*)
	};

	($stream:expr, $type:ident $($tt:tt)*) => {
		parse_error!(context=$crate::stream::Contexted::context($stream).clone(), $type$($tt)*)
	};
}

mod error;
pub mod expression;
pub mod token;
pub mod stream;
pub mod block;

// TODO: change public exports to more minimal.
pub use block::Block;
pub use error::{Error, ErrorType, Result};
pub use token::{Token/*, ParenType, Literal*/};
pub use expression::Expression;
pub use stream::{Stream, Context, Contexted};