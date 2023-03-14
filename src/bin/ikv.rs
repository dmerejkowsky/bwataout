use bwataout::ikv::*;
use chrono::prelude::*;

fn main() -> Result<(), String> {
    let dir = get_ikv_dir_from_args()?;
    let map = parse_map(&dir)?;
    let now: DateTime<Local> = Local::now();
    let trips_md = read_trips_md(&dir, now)?;
    let trips = parse_trips(&trips_md);
    let mut total_distance = 0;
    for trip in trips {
        let distance = traveled_distance(&map, trip.places())?;
        total_distance += distance;
        println!("{distance:2}km - {}", trip.description());
    }
    let year = now.year();
    let month = now.month();
    println!("---\nIKV for {year} {month:0>2} = {total_distance}km");

    Ok(())
}
