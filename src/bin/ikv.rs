use bwataout::ikv::*;

fn main() -> Result<(), String> {
    let md_path = get_md_path()?;
    let display = md_path
        .file_name()
        .ok_or_else(|| "markdown path has no parent")?
        .to_string_lossy();
    println!("{}", display);
    println!("{}", "-".repeat(display.to_string().len()));
    let dir = md_path.parent().expect("md path should have a parent");
    let trips_md = read_trips_md(&md_path)?;
    let trips = parse_trips(&trips_md);
    let map = parse_map(dir)?;
    let mut total_distance = 0;
    for trip in trips {
        let distance = traveled_distance(&map, trip.places())?;
        total_distance += distance;
        println!("{:0>2} {distance:2}km - {}", trip.day(), trip.description());
    }
    println!("---\nIKV = {total_distance}km");

    Ok(())
}
