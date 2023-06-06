use std::path::{Path, PathBuf};
use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

/// A Map is created with an slice of tuples (start, destination, distance)
/// It then can lookup any couple of (start, destination)
#[derive(Debug)]
pub struct Map {
    // Note: Since distances are symetric, we store them as tuple
    // (start, destination) while ensuring that (start, destination) are
    // always in alphabetical order
    distances: HashMap<(String, String), u32>,
}

impl Map {
    /// Create a new map by giving a list of triplets
    /// (start, destination, distance)
    pub fn new(distances: &[(&str, &str, u32)]) -> Self {
        let mut map: HashMap<_, _> = HashMap::new();
        for (start, destination, length) in distances.iter().copied() {
            if start < destination {
                map.insert((start.to_owned(), destination.to_owned()), length);
            } else {
                map.insert((destination.to_owned(), start.to_owned()), length);
            }
        }
        Self { distances: map }
    }

    pub fn lookup(&self, start: &str, destination: &str) -> Option<u32> {
        let key = if start < destination {
            (start.to_owned(), destination.to_owned())
        } else {
            (destination.to_owned(), start.to_owned())
        };
        self.distances.get(&key).copied()
    }
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut distances = vec![];
        for (i, line) in s.split_terminator('\n').enumerate() {
            let lineno = i + 1;
            let split: Vec<_> = line.split_whitespace().collect();
            let n = split.len();
            if n == 0 {
                // Allow blank lines
                continue;
            }
            if n != 4 {
                return Err(format!(
                    "{lineno}: Each line should have 4 words, got {n} instead"
                ));
            }
            let second_word = split[1];
            if second_word != "->" {
                return Err(format!(
                    "{lineno}: Second word should be an '->', got '{second_word}' instead"
                ));
            }
            let start = split[0].trim();
            let end = split[2].trim();
            let distance_str = split[3].trim();
            let distance = match distance_str.parse::<u32>() {
                Ok(v) => v,
                Err(e) => return Err(format!("{lineno}: could not parse distance as u32: {e}")),
            };
            distances.push((start, end, distance));
        }
        Ok(Map::new(&distances))
    }
}

pub fn parse_trip(s: &str) -> Vec<&str> {
    s.split(' ').map(|x| x.trim()).collect()
}

// Compute the traveled distance of a trip
// All pairs of places in the trip must be found in the Map
pub fn traveled_distance(map: &Map, trip: &[&str]) -> Result<u32, String> {
    let mut total = 0;
    for window in trip.windows(2) {
        let [start, destination] =
            <[&str; 2]>::try_from(window).expect("windows should be of size 2");
        match map.lookup(start, destination) {
            Some(value) => total += value,
            None => return Err(format!("{start} -> {destination} not found in map")),
        }
    }
    Ok(total)
}

pub fn get_md_path() -> Result<PathBuf, String> {
    let args: Vec<_> = std::env::args().collect();
    let nargs = args.len() - 1;
    if nargs != 1 {
        return Err(format!("Expecting 1 argument, got {nargs}"));
    };
    let dir = PathBuf::from(&args[1]);
    Ok(dir)
}

pub fn read_trips_md(trips_md: &Path) -> Result<String, String> {
    std::fs::read_to_string(trips_md)
        .map_err(|e| format!("Could not read {}: {e}", trips_md.display()))
}

pub fn parse_map(dir: &Path) -> Result<Map, String> {
    let map_path = dir.join("map.txt");
    let map_txt = std::fs::read_to_string(&map_path)
        .map_err(|e| format!("Could not read {}: {e}", map_path.display()))?;
    map_txt
        .parse::<Map>()
        .map_err(|e| format!("Could not parse {}: {e}", map_path.display()))
}

pub struct Trip<'a> {
    description: &'a str,
    places: Vec<&'a str>,
    day: &'a str,
}

impl<'a> Trip<'a> {
    pub fn description(&self) -> &str {
        self.description
    }

    pub fn places(&self) -> &[&str] {
        self.places.as_ref()
    }

    pub fn day(&self) -> &str {
        self.day
    }
}

pub fn parse_trips(trips_md: &str) -> Vec<Trip> {
    let mut trips = vec![];
    for line in trips_md.split_terminator('\n') {
        if line.starts_with(|c: char| c.is_ascii_digit()) {
            let mut split: VecDeque<_> = line.split_whitespace().collect();
            let day = split.pop_front().expect("A split can nevery be empty");
            trips.push(Trip {
                day,
                description: line,
                places: split.into(),
            });
        }
    }
    trips
}

#[cfg(test)]
mod tests;
