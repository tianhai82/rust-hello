use std::collections::HashMap;
fn main() {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().expect("key not provided");
    let value = arguments.next().unwrap();
    print!("{}\t{}\n", key, value);
    // std::fs::write("kv.db", content).unwrap();

    let mut database = Database::new().expect("Database::new crashed");
    database.insert(key.to_uppercase(), value);
    database.insert(key, value);
    
    // println!(
    //     "the key is '{}' and the value is '{}'",
    //     key.expect("key was not there"),
    //     value.expect("value was not there")
    // );
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        // let content = match std::fs::read_to_string("kv.db") {
        //     Ok(c) => c,
        //     Err(error) => {
        //         return Err(error);
        //     },
        // };
        let mut map = HashMap::new();
        let content = std::fs::read_to_string("kv.db")?; // this is equivalent to the above code
        for line in content.lines() {
            // let pair = line.split_once('\t').expect("database corrupted");
            // let key = pair.0;
            // let value = pair.1;
            let (key, value) = line.split_once('\t').expect("database corrupted");
            map.insert(key.to_owned(), value.to_owned());
        }
        Ok(Database { map: map })
    }

    fn insert(&mut self, key: &str, value: &str) {
        self.map.insert(key, value);
    }
}
