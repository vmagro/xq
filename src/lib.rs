use graphql_parser::query::Query;
use graphql_parser::query::{Selection, SelectionSet};
use serde_json::json;
use serde_json::value::Value;

fn extract_selection_set(sel: &SelectionSet, data: &Value) -> Value {
    let mut res = serde_json::map::Map::new();
    for sel in &sel.items {
        match sel {
            Selection::Field(f) => {
                let dst_name = match &f.alias {
                    Some(a) => a,
                    None => &f.name,
                };
                println!("Selecting field {} as {}", f.name, dst_name);
                let val = match data.get(f.name.clone()) {
                    Some(v) => v,
                    None => &json!(null),
                };
                if !f.selection_set.items.is_empty() {
                    let val = extract_selection_set(&f.selection_set, val);
                }
                        res.insert(dst_name.to_string(), val.clone());
            }
            _ => {
                panic!("Unsupported selection type: {:?}", sel);
            }
        }
    }
    json!(res)
}

pub fn eval(q: &Query, data: &Value) -> Value {
    return extract_selection_set(&q.selection_set, data);
}
