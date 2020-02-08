use graphql_parser::parse_query;
use graphql_parser::query::Query;
use graphql_parser::query::{Definition, OperationDefinition, Selection};
use serde_json::json;
use serde_json::value::Value;

fn main() {
    // let doc = roxmltree::Document::parse(std::include_str!("example.xml")).unwrap();
    // let xml_root = doc.root_element();
    let json_root = serde_json::from_str(std::include_str!("example.json")).unwrap();

    // let ast = parse_query("query { books { author } }").unwrap();
    let ast = parse_query("query { top_level, not_a_field}").unwrap();
    if ast.definitions.len() != 1 {
        panic!("Must have exactly 1 definition");
    }
    let query_root = &ast.definitions[0];
    match query_root {
        Definition::Operation(OperationDefinition::Query(q)) => {
            let res = eval_query(q, json_root);
            println!("{}", res.to_string())
        }
        _ => {
            panic!("Unsupported root: {:?}", query_root);
        }
    }
}

fn eval_query(q: &Query, data: Value) -> Value {
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
