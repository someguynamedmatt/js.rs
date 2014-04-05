use js::value::{Value, VNumber, VInteger, VFunction, VObject, ResultValue};
use js::function::Function;
use std::io::stdio;
use collections::treemap::TreeMap;
use js::object::ObjectData;
use std::f64;
use std::gc::Gc;

/// Get the absolute value of a number
pub fn abs(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(VNumber(if args.len() >= 1 {
		args.get(0).to_num().abs()
	} else {
		f64::NAN
	}))
}
/// Get the arccos of a number
pub fn acos(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(VNumber(if args.len() >= 1 {
		args.get(0).to_num().acos()
	} else {
		f64::NAN
	}))
}
/// Get the arcsine of a number
pub fn asin(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(VNumber(if args.len() >= 1 {
		args.get(0).to_num().asin()
	} else {
		f64::NAN
	}))
}
/// Get the arctangent of a number
pub fn atan(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(VNumber(if args.len() >= 1 {
		args.get(0).to_num().atan()
	} else {
		f64::NAN
	}))
}
/// Get the arctangent of a numbers
pub fn atan2(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(VNumber(if args.len() >= 1 {
		f64::atan2(args.get(0).to_num(), args.get(1).to_num())
	} else {
		f64::NAN
	}))
}
/// Get the cubic root of a number
pub fn cbrt(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(VNumber(if args.len() >= 1 {
		args.get(0).to_num().cbrt()
	} else {
		f64::NAN
	}))
}
/// Get lowest integer above a number
pub fn ceil(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(VNumber(if args.len() >= 1 {
		args.get(0).to_num().ceil()
	} else {
		f64::NAN
	}))
}
/// Get the cosine of a number
pub fn cos(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(VNumber(if args.len() >= 1 {
		args.get(0).to_num().cos()
	} else {
		f64::NAN
	}))
}
/// Get the highest integer below a number
pub fn floor(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(VNumber(if args.len() >= 1 {
		args.get(0).to_num().floor()
	} else {
		f64::NAN
	}))
}
/// Get the sine of a number
pub fn sin(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(VNumber(if args.len() >= 1 {
		args.get(0).to_num().sin()
	} else {
		f64::NAN
	}))
}
/// Get the tangent of a number
pub fn tan(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	Ok(VNumber(if args.len() >= 1 {
		args.get(0).to_num().tan()
	} else {
		f64::NAN
	}))
}
/// Create a new 'Math' object
pub fn _create() -> Value {
	let mut math = TreeMap::new();
	math.insert(~"E", VNumber(f64::consts::E));
	math.insert(~"LN2", VNumber(f64::consts::LN_2));
	math.insert(~"LN10", VNumber(f64::consts::LN_10));
	math.insert(~"LOG2E", VNumber(f64::consts::LOG2_E));
	math.insert(~"LOG10E", VNumber(f64::consts::LOG10_E));
	math.insert(~"SQRT1_2", VNumber(0.5f64.sqrt()));
	math.insert(~"SQRT2", VNumber(f64::consts::SQRT2));
	math.insert(~"PI", VNumber(f64::consts::PI));
	math.insert(~"abs", VFunction(Gc::new(Function::new(abs, 1))));
	math.insert(~"acos", VFunction(Gc::new(Function::new(acos, 1))));
	math.insert(~"asin", VFunction(Gc::new(Function::new(asin, 1))));
	math.insert(~"atan", VFunction(Gc::new(Function::new(atan, 1))));
	math.insert(~"atan2", VFunction(Gc::new(Function::new(atan2, 2))));
	math.insert(~"cbrt", VFunction(Gc::new(Function::new(cbrt, 1))));
	math.insert(~"ceil", VFunction(Gc::new(Function::new(ceil, 1))));
	math.insert(~"cos", VFunction(Gc::new(Function::new(cos, 1))));
	math.insert(~"floor", VFunction(Gc::new(Function::new(floor, 1))));
	math.insert(~"sin", VFunction(Gc::new(Function::new(sin, 1))));
	math.insert(~"tan", VFunction(Gc::new(Function::new(tan, 1))));
	VObject(Gc::new(math))
}