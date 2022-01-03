fn main() {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next();
    let value = arguments.next();
    let content = format!("{}\t{}\n", key.expect("key not provided"), value.unwrap());
    let write_result = std::fs::write("kv.db", content);

    // println!(
    //     "the key is '{}' and the value is '{}'",
    //     key.expect("key was not there"),
    //     value.expect("value was not there")
    // );
}
