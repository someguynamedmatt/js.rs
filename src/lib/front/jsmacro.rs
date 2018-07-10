#[macro_export]
macro_rules! js(
	($global:expr, { $( $name:tt: $value:tt ),* }) => ({
		let value = Value::new_obj(Some($global));
		$(
			value.set_field($name, js!($value));
		)+
		value
	});
	($inp:expr) => (
		to_value($inp)
	);
);
#[macro_export]
macro_rules! js_extend(
	($object:expr, { $name:tt: $value:tt }) => (
		$object.set_field($name, $value)
	);
	($object:expr, {
		$($name:tt, $value:tt),+
	}) => ({
		let object = $object;
		$(
			object.set_field($name, js!($value));
		)+
	});
);
