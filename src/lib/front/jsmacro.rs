#[macro_export]
macro_rules! js(
	($global:expr, {
        $( $name:tt: $value:expr ),+
    }) => ({
		let value = Value::new_obj(Some($global));
		$(
			value.set_field($name, js!($value));
		)*
		value
	});
	($global:expr, {
        $name:tt: $value:expr
    }) => ({
		let value = Value::new_obj(Some($global));
        value.set_field($name, js!($value));
		value
	});
	($inp:expr) => (
		to_value($inp)
	);
);
#[macro_export]
macro_rules! js_extend(
	($object:expr, { $name:tt: $value:expr }) => (
		$object.set_field($name, $value)
	);
	($object:expr, {
		$($name:tt: $value:expr),+
	}) => ({
		let object = $object;
		$(
			object.set_field($name, js!($value));
		)*
	});
);
