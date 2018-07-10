use front::stdlib::value::{Value, ResultValue, ToValue, FromValue, to_value, from_value};
use front::stdlib::function::Function;
use std::collections::btree_map::BTreeMap;
pub static PROTOTYPE: &'static str = "prototype";
pub static INSTANCE_PROTOTYPE: &'static str = "__proto__";
//#[derive(Clone)]
pub type ObjectData = BTreeMap<String, Property>;

#[derive(Clone)]
/// A Javascript property
pub struct Property {
    /// If the type of this can be changed and this can be deleted
    pub configurable : bool,
    /// If the property shows up in enumeration of the object
    pub enumerable: bool,
    /// If this property can be changed with an assignment
    pub writable: bool,
    /// The value associated with the property
    pub value: Value,
    /// The function serving as getter
    pub get: Value,
    /// The function serving as setter
    pub set: Value
}
impl Property {
    /// Make a new property with the given value
    pub fn new(value : Value) -> Property {
        Property {
            configurable: false,
            enumerable: false,
            writable: false,
            value: value,
            get: Value::undefined(),
            set: Value::undefined()
        }
    }
}

impl ToValue for Property {
    fn to_value(&self) -> Value {
        let prop = Value::new_obj(None);
        prop.set_field("configurable", to_value(self.configurable));
        prop.set_field("enumerable", to_value(self.enumerable));
        prop.set_field("writable", to_value(self.writable));
        prop.set_field("value", self.value);
        prop.set_field("get", self.get);
        prop.set_field("set", self.set);
        prop
    }
}
impl FromValue for Property {
    fn from_value(v:Value) -> Result<Property, &'static str> {
        Ok(Property {
            configurable: from_value(v.get_field("configurable")).unwrap(),
            enumerable: from_value(v.get_field("enumerable")).unwrap(),
            writable: from_value(v.get_field("writable")).unwrap(),
            value: v.get_field("value"),
            get: v.get_field("get"),
            set: v.get_field("set")
        })
    }
}
/// Create a new object
pub fn make_object(_:Vec<Value>, _:Value, _:Value, _:Value) -> ResultValue {
    Ok(Value::undefined())
}
/// Get the prototype of an object
pub fn get_proto_of(args:Vec<Value>, _:Value, _:Value, _:Value) -> ResultValue {
    let obj = args[0];
    Ok(obj.get_field(INSTANCE_PROTOTYPE))
}
/// Set the prototype of an object
pub fn set_proto_of(args:Vec<Value>, _:Value, _:Value, _:Value) -> ResultValue {
    let obj = args[0];
    let proto = args[1];
    obj.set_field(INSTANCE_PROTOTYPE, proto);
    Ok(obj)
}
/// Define a property in an object
pub fn define_prop(args:Vec<Value>, _:Value, _:Value, _:Value) -> ResultValue {
    let obj = args[0];
    let prop = from_value::<String>(args[1]).unwrap();
    let desc = from_value::<Property>(args[2]).unwrap();
    obj.set_prop(prop.as_slice(), desc);
    Ok(Value::undefined())
}
/// To string
pub fn to_string(_:Vec<Value>, _:Value, _:Value, this:Value) -> ResultValue {
    Ok(to_value(this.to_string()))
}
/// Check if it has a property
pub fn has_own_prop(args:Vec<Value>, _:Value, _:Value, this:Value) -> ResultValue {
    let prop = if args.len() == 0 {
        None
    } else {
        from_value::<String>(args[0]).ok()
    };
    Ok(to_value(prop.is_some() && this.get_prop(prop.unwrap().as_slice()).is_some()))
}
/// Create a new `Object` object
pub fn _create(global:Value) -> Value {
    let object = Function::make(make_object, []);
    let prototype = js!(global, {
        "hasOwnProperty": Function::make(has_own_prop, ["property"]),
        "toString": Function::make(to_string, [])
    });
    js_extend!(object, {
        "length": i32,
        PROTOTYPE: prototype,
        "setPrototypeOf": Function::make(get_proto_of, ["object", "prototype"]),
        "getPrototypeOf": Function::make(get_proto_of, ["object"]),
        "defineProperty": Function::make(define_prop, ["object", "property"])
    });
    object
}
/// Initialise the `Object` object on the global object
pub fn init(global:Value) {
    js_extend!(global, {
        "Object": _create(global)
    });
}
