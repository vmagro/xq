use graphql_parser::query::Query;
use graphql_parser::query::{Selection};
use serde_json::json;
use serde_json::value::Value;

pub fn eval(q: &Query, data: Value) -> Value {
    let mut res = serde_json::map::Map::new();
    for sel in &q.selection_set.items {
        match sel {
            Selection::Field(f) => {
                let dst_name = match &f.alias {
                    Some(a) => a,
                    None => &f.name,
                };
                println!("Selecting field {} as {}", f.name, dst_name);
                let val = data.get(f.name.clone());
                match val {
                    Some(v) => {
                        res.insert(dst_name.to_string(), v.clone());
                    }
                    None => {
                        res.insert(dst_name.to_string(), json!(null));
                    }
                };
            }
            _ => {
                panic!("Unsupported selection type: {:?}", sel);
            }
        }
    }
    json!(res)
}
