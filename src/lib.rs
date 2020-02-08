use graphql_parser::query::Query;
use graphql_parser::query::{Selection, SelectionSet};
use serde_json::json;
use serde_json::value::Value;
use serde_json::value::Value::{Array, Object, Null, String};

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
                if !f.arguments.is_empty() {
                    panic!("Cannot extract field with arguments yet");
                }
                let val = match data.get(f.name.clone()) {
                    Some(v) => v,
                    None => &json!(null),
                };
                match val {
                    Object(_) => {
                        let obj = extract_selection_set(&f.selection_set, val);
                        res.insert(dst_name.to_string(), obj);
                    },
                    Array(arr) => {
                        // let mut res: Vec<Value> = Vec::new();
                        let mut items = vec![];
                        for elem in arr {
                            let val = extract_selection_set(&f.selection_set, elem);
                            items.push(val);
                        }
                        res.insert(dst_name.to_string(), items.into());
                    },
                    Null => {
                        res.insert(dst_name.to_string(), json!(null));
                    }
                    // String(s) => {
                    //     res.insert(dst_name.to_string(), json!(null));
                    // },
                    _ => {
                        res.insert(dst_name.to_string(), val.clone());
                    }
                };
                // res.insert(dst_name.to_string(), field_res);
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
