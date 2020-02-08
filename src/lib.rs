use graphql_parser::query::{Field, Query, Selection, SelectionSet};
use serde_json::json;
use serde_json::value::Value;
use serde_json::value::Value::{Array, Null, Object, String};

fn resolve(field: &Field, data: &Value) -> Value {
    if !field.arguments.is_empty() {
        panic!("Cannot extract field with arguments yet");
    }
    println!("resolving {:?} with {:?}", data, field);
    let val = match data.get(field.name.clone()) {
        Some(v) => v,
        None => &json!(null),
    };
    match val {
        Object(obj) => {
            let mut res_obj = serde_json::map::Map::new();
            for sel in &field.selection_set.items {
                match sel {
                    Selection::Field(f) => {
                        let field_val = obj.get(&f.name);
                        let dst_name = match &f.alias {
                            Some(a) => a,
                            None => &f.name,
                        };
                        res_obj.insert(
                            dst_name.clone(),
                            match field_val {
                                Some(v) => resolve(&f, v),
                                None => json!(null),
                            },
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
                items.push(resolve(&field, &elem));
            }
            json!(items)
        }
        // Null => {
        //     res.insert(dst_name.to_string(), json!(null));
        // }
        // String(s) => {
        //     res.insert(dst_name.to_string(), json!(null));
        // },
        _ => {
            val.clone()
            // res.insert(dst_name.to_string(), val.clone());
        }
    }
}

fn extract_selection_set(sel: &SelectionSet, data: &Value) -> Value {
    let mut res = serde_json::map::Map::new();
    for sel in &sel.items {
        match sel {
            Selection::Field(f) => {
                let dst_name = match &f.alias {
                    Some(a) => a,
                    None => &f.name,
                };
                res.insert(dst_name.to_string(), resolve(&f, data));
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
