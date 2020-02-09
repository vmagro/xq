use std::env;
use std::io::Read;

use graphql_parser::parse_query;
use graphql_parser::query::{Definition, OperationDefinition};

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];

    let ast = parse_query(query).unwrap();
    if ast.definitions.len() != 1 {
        panic!("Must have exactly 1 definition");
    }
    let query_root = &ast.definitions[0];

    let mut buffer = String::new();
    let mut stdin = std::io::stdin();
    stdin.read_to_string(&mut buffer).unwrap();

    let json = serde_json::from_str(&buffer);
    let doc = roxmltree::Document::parse(&buffer);

    let root;

    if let Ok(json) = json {
        root = json;
    } else if let Ok(doc) = doc {
        root = xq::xml_to_json(doc.root_element());
        // println!("{}", serde_json::to_string_pretty(&root).unwrap());
    } else {
        panic!("Input is neither valid JSON nor XML");
    }

    match query_root {
        Definition::Operation(OperationDefinition::Query(q)) => {
            let res = xq::eval(q, &root);
            println!("{}", serde_json::to_string_pretty(&res).unwrap());
        }
        _ => {
            panic!("Unsupported root: {:?}", query_root);
        }
    }
}
