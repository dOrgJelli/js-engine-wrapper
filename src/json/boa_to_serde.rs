use polywrap_wasm_rs::{ JSON };
use boa_engine::{
    JsValue,
    JsResult,
    Context,
    property::{ PropertyKey }
};

pub fn boa_to_serde(value: &JsValue, context: &mut Context) -> JsResult<JSON::Value> {
    match value {
        JsValue::Null => Ok(JSON::Value::Null),
        JsValue::Undefined => Ok(JSON::Value::Null),
        &JsValue::Boolean(b) => Ok(b.into()),
        JsValue::String(string) => Ok(string.as_str().into()),
        &JsValue::Rational(rat) => Ok(rat.into()),
        &JsValue::Integer(int) => Ok(int.into()),
        JsValue::BigInt(bigint) => Ok(bigint.to_string().as_str().into()),
        JsValue::Object(obj) => {
            if obj.is_array() {
                let len = obj.get("length", context)?.to_length(context).unwrap();
                // let len = obj.length_of_array_like(context)?;
                let mut arr = Vec::with_capacity(len);

                let obj = obj.borrow();

                for k in 0..len as u32 {
                    let val = obj.properties().get(&k.into()).map_or(JsValue::Null, |desc| {
                        desc.value().cloned().unwrap_or(JsValue::Null)
                    });
                    arr.push(val.to_json(context)?);
                }

                Ok(JSON::Value::Array(arr))
            } else {
                let mut map = JSON::Map::new();
                for (key, property) in obj.borrow().properties().iter() {
                    let key = match &key {
                        PropertyKey::String(string) => string.as_str().to_owned(),
                        PropertyKey::Index(i) => i.to_string(),
                        PropertyKey::Symbol(_sym) => {
                            return context.throw_type_error("cannot convert Symbol to JSON")
                        }
                    };

                    let value = match property.value() {
                        Some(val) => val.to_json(context)?,
                        None => JSON::Value::Null,
                    };

                    map.insert(key, value);
                }

                Ok(JSON::Value::Object(map))
            }
        }
        JsValue::Symbol(_sym) => context.throw_type_error("cannot convert Symbol to JSON"),
    }
}
