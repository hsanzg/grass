use std::collections::HashMap;

use num_traits::cast::ToPrimitive;

use super::Builtin;
use crate::unit::Unit;
use crate::value::{Number, Value};

pub(crate) fn register(f: &mut HashMap<String, Builtin>) {
    f.insert(
        "length".to_owned(),
        Box::new(|args, _| {
            max_args!(args, 1);
            let len = match arg!(args, 0, "list") {
                Value::List(v, _) => Number::from(v.len()),
                _ => Number::from(1),
            };
            Ok(Value::Dimension(len, Unit::None))
        }),
    );
    f.insert(
        "nth".to_owned(),
        Box::new(|args, _| {
            max_args!(args, 2);
            let list = match arg!(args, 0, "list") {
                Value::List(v, _) => v,
                _ => return Err("Missing argument $list.".into()),
            };
            let n = match arg!(args, 1, "n") {
                Value::Dimension(num, _) => num,
                v => return Err(format!("$n: {} is not a number.", v).into()),
            };

            if n == Number::from(0) {
                return Err("$n: List index may not be 0.".into());
            }

            if n.abs() > Number::from(list.len()) {
                return Err(format!(
                    "$n: Invalid index {} for a list with {} elements.",
                    n,
                    list.len()
                )
                .into());
            }

            if n.is_decimal() {
                return Err(format!("$n: {} is not an int.", n).into());
            }

            if n > Number::from(0) {
                Ok(list[n.to_integer().to_usize().unwrap() - 1].clone())
            } else {
                Ok(list[list.len() - n.abs().to_integer().to_usize().unwrap()].clone())
            }
        }),
    );
}
