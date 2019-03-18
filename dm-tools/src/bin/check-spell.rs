use std::io::BufRead;

fn main() {
    let lang = "en_US";

    println!("Enchant version {}\n", enchant::version());

    let mut broker = enchant::Broker::new();

    let providers = broker.list_providers();
    for provider in providers {
        println!("{:?}", provider);
    }

    let dict = broker.request_dict(lang).unwrap();

    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        for word in line.split(" ") {
            let checked = dict.check(word).unwrap();
            if !checked {
                println!("{}", word)
            }

        }
    }
}
