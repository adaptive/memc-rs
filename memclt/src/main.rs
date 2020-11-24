use memcache::Client;
use std::collections::HashMap;
use std::str;

fn main() {
    let mut client = memcache::Client::connect(
        "memcache://127.0.0.1:11211?timeout=10&tcp_nodelay=true&protocol=binary",
    )
    .unwrap();
    client.set("foo", "test", 50).unwrap();

    let result: HashMap<String, (Vec<u8>, u32, Option<u64>)> = client.gets(&["foo"]).unwrap();
    let (key, val, cas) = result.get("foo").unwrap();

    println!(
        "Foo: {:?} {} {}",
        str::from_utf8(key).unwrap(),
        val,
        cas.unwrap()
    );

    client.set("foo", "3000", 50).unwrap();

    println!(
        "Foo: {:?} {} {}",
        str::from_utf8(key).unwrap(),
        val,
        cas.unwrap()
    );
}