#![allow(clippy::module_inception, clippy::missing_const_for_fn)]

/// Setup the quest parser. This should be run before anything within `quest_parser` is used.
pub fn initialize() {
	use quest_core::{Object, types::{ObjectType, RustFn, Text, Kernel, rustfn::Binding}};
	use crate::expression::Executable;

	Kernel::mapping().set_attr_lit("Block", Block::mapping())
		.expect("couldn't defined Block");

	Text::mapping().set_value_lit("eval", RustFn::new("Text::eval", |this, args| {
		fn execute_text(text: String) -> quest_core::Result<Object> {
			Expression::parse_stream(stream::BufStream::from(text).tokens())
				.map_err(|err| err.to_string())?
				.execute()
				.map_err(Into::into)
		}

		this.try_downcast_and_then(|this: &Text| {
			if let Ok(binding) = args.arg(0) {
				Binding::new_stackframe(Some(binding.clone()), args, |_| execute_text(this.to_string()))
			} else {
				execute_text(this.to_string())
			}
		})
	})).expect("couldn't define `eval`");
}

#[macro_use]
mod macros;
mod error;
pub mod expression;
pub mod token;
pub mod stream;
pub mod block;

// TODO: change public exports to more minimal.
pub use block::Block;
pub use error::{Error, ErrorType, Result};
pub use token::Token;
pub use expression::Expression;
pub use stream::{Stream, Context, Contexted};
