use bwataout::ikv;
use chrono::prelude::*;
use std::path::PathBuf;

fn main() -> Result<(), String> {
    let args: Vec<_> = std::env::args().collect();
    let nargs = args.len();
    if nargs != 2 {
        return Err(format!("Expecting 1 argument, got {nargs}"));
    };
    let dir = PathBuf::from(&args[1]);
    let now: DateTime<Local> = Local::now();
    let year = now.year();
    let month = now.month();
    let map_path = dir.join("map.txt");
    let map_str = match std::fs::read_to_string(&map_path) {
        Ok(s) => s,
        Err(e) => return Err(format!("Could not read {}: {e}", map_path.display())),
    };
    let map = match map_str.parse::<ikv::Map>() {
        Ok(m) => m,
        Err(e) => return Err(format!("Could not parse map {}: {e}", map_path.display())),
    };
    let trip_path = dir.join(format!("{}-{:0>2}.md", year, month));
    let trips_md = match std::fs::read_to_string(&trip_path) {
        Ok(s) => s,
        Err(e) => return Err(format!("Could not read {}: {e}", trip_path.display())),
    };
    println!("{trips_md}");
    let mut in_fence = false;
    let mut trips = vec![];
    for (i, line) in trips_md.split_terminator('\n').enumerate() {
        if line.is_empty() {
            continue;
        }
        let lineno = i + 1;
        if in_fence && line != "```" {
            if !line.contains(':') {
                return Err(format!("{}:{} Missing ':'", trip_path.display(), lineno));
            }
            let split: Vec<_> = line.split(':').collect();
            let trip = ikv::parse_trip(split[1]);
            trips.push(trip);
        }
        if line == "```text" {
            in_fence = true;
        }
        if line == "```" {
            in_fence = false;
        }
    }

    let mut total = 0;

    for trip in trips.iter() {
        let d = ikv::traveled_distance(&map, trip)?;
        total += d;
    }

    println!("IKV for {year} {month:0>2} = {total}km");

    Ok(())
}
