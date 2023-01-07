use std::{collections::HashMap, str::FromStr};

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
    s.split("->").map(|x| x.trim()).collect()
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

#[cfg(test)]
mod tests;
