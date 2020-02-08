use graphql_parser::query::{Field, Query, Selection, SelectionSet};
use serde_json::json;
use serde_json::value::Value;
use serde_json::value::Value::{Array, Object};

fn resolve_selset(selection_set: &SelectionSet, data: &Value) -> Value {
    match data {
        Object(_) => {
            let mut res_obj = serde_json::map::Map::new();
            for sel in &selection_set.items {
                match sel {
                    Selection::Field(f) => {
                        let dst_name = match &f.alias {
                            Some(a) => a,
                            None => &f.name,
                        };
                        res_obj.insert(
                            dst_name.clone(),
                            resolve_field(&f, data),
                        );
                    }
                    _ => panic!("Cannot process a selection that is not a field"),
                }
            }
            json!(res_obj)
        }
        Array(arr) => {
            // let mut res: Vec<Value> = Vec::new();
            let mut items = vec![];
            for elem in arr {
                println!("resolving object in array {:?}", elem);
                items.push(resolve_selset(selection_set, &elem));
            }
            json!(items)
        }
        _ => {
            data.clone()
        }
    }
}

fn resolve_field(field: &Field, data: &Value) -> Value {
    if !field.arguments.is_empty() {
        panic!("Cannot extract field with arguments yet");
    }
    let val = match data.get(field.name.clone()) {
        Some(v) => v,
        None => &json!(null),
    };
    resolve_selset(&field.selection_set, val)
}

pub fn eval(q: &Query, data: &Value) -> Value {
    return resolve_selset(&q.selection_set, data);
}
