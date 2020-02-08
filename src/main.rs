use graphql_parser::parse_query;
use graphql_parser::query::{Definition, OperationDefinition, Selection};

fn main() {
    println!("Hello, world!");
    let ast = parse_query("query { root1, root2 { nested1 }, aliased: root3 }").unwrap();
    if ast.definitions.len() != 1 {
        panic!("Must have exactly 1 definition");
    }
    let root = &ast.definitions[0];
    match root {
        Definition::Operation(OperationDefinition::Query(q)) => {
            for sel in &q.selection_set.items {
                match sel {
                    Selection::Field(f) => {
                        println!("Selecting field {}", f.name);
                    }
                    _ => {
                        panic!("Unsupported root: {:?}", root);
                    }
                }
            }
        }
        _ => {
            panic!("Unsupported root: {:?}", root);
        }
    }
}
