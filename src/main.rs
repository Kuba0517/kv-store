use std::collections::HashMap;
use std::io;

fn main() {
    let mut store: HashMap<String, String> = HashMap::new();
    store.insert("hello".to_string(), "world".to_string());
    println!("{:#?}", store.get("hello"));

    loop {
        let mut buffer = String::new();

        io::stdin().read_line(&mut buffer).unwrap();

        buffer = buffer.strip_suffix("\n").unwrap().to_string();

        let options: Vec<&str> = buffer.split(" ").collect();

        if options[0] == "GET" {
            println!("{:#?}", store.get(options[1]));
        } else if options[0] == "SET" {
            store.insert(options[1].to_string(), options[2].to_string());
        }
    }
}
