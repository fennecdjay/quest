#![allow(unused)]

use crate::obj::types::ObjectType;

mod obj;
mod parse;

fn main() {
	// let x = [0xff]
	let mut stream = parse::Stream::from_str(r##"
		true
		#"x" = 1 + 2;
 	"##);

	let mut stream = stream.collect::<parse::Result<Vec<_>>>().unwrap().into_iter();

	let o = obj::Object::from("a");
	// println!("{:#?}", obj::types::Number::mapping());
	// return;
	// println!("{:#?}", o);
	println!("{:?}", o.get_attr(&"__parent__".into()).unwrap()
			.get_attr(&"name".into())
	);
	return;
	println!("{:?}", o.get_attr(&"true".into()));
	let expression = parse::Expression::try_from_iter(&mut stream).unwrap();
	println!("{:#?}", expression.execute_default());
}

//	let mut stream = parse::Stream::from_str(r##"

// "Frac" = {
// 	"()" = {
// 		"numer" = (_1."@num"());
// 		"denom" = (_2."@num"());
// 		if((_2 == 0), {
// 			return(-2, "error!")
// 		})();
// 		__this
// 	};

// 	"@text" = {
// 		__this."numer" + "/" + __this."denom"
// 	};
// }();

// "half" = Frac(1, 2);
// disp((half."@text")() + " = half");
// 	"##);

/*

		# a."+@"()
		# + += +@ - -= -@ * *= ** **= % %= / /= ! != = ==
		# < <= <=> << <<= > >= >> >>= ~ & &= && | |= || ^ ^= . .= .~ , ;
#// (4 + (5 * 3)) * 3;
#// (12."floor")(x);
#// "y" = ((1 ** 2) * 3) + 4;
#// 3 + { (x), (y); (z) };
#// "car" = { "x" = [_1, _2]."last"[]; (_1 * _2)."floor"(x) };
#// this.x = "123" + that.34; # this
#// foo = { _1 * (_2.'3' = _3) };
#// disp("hello there," + this.x);
*/