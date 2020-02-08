use graphql_parser::parse_query;

fn main() {
    println!("Hello, world!");
    let ast = parse_query("query { field1, field2 }").unwrap();
    println!("{:?}", ast);
}
