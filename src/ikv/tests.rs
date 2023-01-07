use super::*;

#[test]
fn test_lookup_distance_in_map() {
    let map = Map::new(&[
        ("home", "arolla", 12),
        ("home", "engie", 15),
        ("engie", "arolla", 10),
    ]);

    let distance = map.lookup("home", "arolla").unwrap();
    assert_eq!(distance, 12);
    let distance = map.lookup("arolla", "home").unwrap();
    assert_eq!(distance, 12);

    let missing = map.lookup("home", "bnp");
    assert!(missing.is_none());
}

#[test]
fn map_parsing_ok() {
    let map_str = "\
home -> arolla 12
home -> engie 15
home -> bnp 13
        ";
    let map: Map = map_str.parse().unwrap();
    assert_eq!(map.lookup("home", "engie").unwrap(), 15);
}

#[test]
fn map_parsing_not_enough_words() {
    let map_str = "\
home -> arolla 12
home -> engie
home -> bnp 13
        ";
    map_str.parse::<Map>().unwrap_err();
}

#[test]
fn test_compute_traveled_distance_happy() {
    let trip = &["home", "arolla", "home"];
    let map = Map::new(&[("home", "arolla", 12)]);

    let traveled_distance = traveled_distance(&map, trip).unwrap();
    assert_eq!(traveled_distance, 24);
}

#[test]
fn test_compute_traveled_distance_missing_places_in_map() {
    let trip = &["home", "arolla", "engie", "home"];
    #[rustfmt::skip]
    let map = Map::new(&[
        ("home", "arolla", 12),
        ("arolla", "engie", 15),
        ("home", "engie", 20),
    ]);

    let traveled_distance = traveled_distance(&map, trip).unwrap();
    assert_eq!(traveled_distance, 47);
}

#[test]
fn test_parse_trip_ok() {
    let trip_str = "a -> b -> c";
    let places = parse_trip(trip_str);
    assert_eq!(places, vec!["a", "b", "c"]);
}
