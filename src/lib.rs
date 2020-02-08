use std::collections::HashMap;

use graphql_parser::query::{Field, Query, Selection, SelectionSet};
use inflector::string::pluralize::to_plural;
use serde_json::json;
use serde_json::value::Value;
use serde_json::value::Value::{Array, Object};

pub fn xml_to_json(elem: roxmltree::Node) -> Value {
    match elem.text() {
        Some(text) => {
            // check if there are attributes
            if !elem.attributes().is_empty() {
                let mut map = serde_json::Map::new();
                for attrib in elem.attributes() {
                    map.insert(format!("@{}", attrib.name()), json!(attrib.value()));
                }
                map.insert("text".to_string(), json!(text));
                json!(map)
            } else {
                json!(text)
            }
        }
        None => {
            if elem.has_children() {
                let mut children = HashMap::new();
                for child in elem.children() {
                    let tag = child.tag_name().name();
                    if !children.contains_key(tag) {
                        children.insert(tag, vec![]);
                    }
                    let c = children.get_mut(tag).unwrap();
                    c.push(xml_to_json(child));
                }
                let mut obj = serde_json::Map::new();
                for (key, val) in children {
                    // uniquely tagged children get converted to direct properties
                    // children that have more than one
                    if val.len() == 1 {
                        obj.insert(key.to_string(), val[0].clone());
                    } else {
                        obj.insert(to_plural(key), Value::Array(val));
                    }
                }
                return json!(obj);
            }
            panic!("element has no children or text");
            // json!(null)
        }
    }
}

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
                        res_obj.insert(dst_name.clone(), resolve_field(&f, data));
                    }
                    _ => panic!("Cannot process a selection that is not a field"),
                }
            }
            json!(res_obj)
        }
        Array(arr) => {
            let mut items = vec![];
            for elem in arr {
                items.push(resolve_selset(selection_set, &elem));
            }
            json!(items)
        }
        _ => data.clone(),
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
