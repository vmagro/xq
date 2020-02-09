use std::collections::HashMap;

use graphql_parser::query::{Field, Query, Selection, SelectionSet};
use inflector::string::pluralize::to_plural;
use serde_json::json;
use serde_json::value::Value;
use serde_json::value::Value::{Array, Object};

pub fn xml_to_json(elem: roxmltree::Node) -> Value {
    let mut children = HashMap::new();
    for child in elem.children() {
        if child.is_text() {
            continue;
        }
        let tag = match child.tag_name().namespace() {
            Some(ns) => format!("{}:{}", ns, child.tag_name().name()),
            None => child.tag_name().name().to_string(),
        };
        let json_child = xml_to_json(child);
        children.entry(tag).and_modify(|v: &mut Vec<Value>| { v.push(xml_to_json(child)) }).or_insert(vec![json_child]);
    }
    let mut obj = serde_json::Map::new();
    for (key, val) in &children {
        // uniquely tagged children get converted to direct properties
        // children that have more than one
        if val.len() == 1 {
            obj.insert(key.to_string(), val[0].clone());
        } else {
            obj.insert(to_plural(key), Value::Array(val.to_vec()));
        }
    }
    for attrib in elem.attributes() {
        obj.insert(format!("@{}", attrib.name()), json!(attrib.value()));
    }
    // handle otherwise-empty objects
    if elem.attributes().is_empty() && children.is_empty() {
        return match elem.text() {
            Some(text) => json!(text),
            None => json!(null),
        };
    }
    match elem.text() {
        Some(text) => {
            obj.insert("text".to_string(), json!(text));
        }
        None => (),
    };
    return json!(obj);
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
