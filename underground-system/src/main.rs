use std::collections::HashMap;

type Timestamp = i32;
type Interval = i32;
type CustomerId = i32;

mod search {
    use crate::{Trip, UndergroundSystem};

    pub fn trips_by_stations(
        usys: &UndergroundSystem,
        start_station: String,
        end_station: String,
    ) -> Vec<&Trip> {
        usys.trips
            .iter()
            .filter(|t| t.start_station == start_station && t.end_station == end_station)
            .collect()
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Check {
    station_name: String,
    t: Timestamp,
}

impl Check {
    fn new(station_name: String, t: Timestamp) -> Self {
        Check { station_name, t }
    }
}

#[derive(Debug)]
pub struct Trip {
    travel_time: Interval,
    start_station: String,
    end_station: String,
}

impl Trip {
    fn new(travel_time: Interval, start_station: String, end_station: String) -> Trip {
        Trip {
            travel_time,
            start_station,
            end_station,
        }
    }
}

pub struct UndergroundSystem {
    ins: HashMap<CustomerId, Check>,
    trips: Vec<Trip>,
}

impl UndergroundSystem {
    fn new() -> Self {
        let trips = vec![];
        let ins = HashMap::new();
        UndergroundSystem { trips, ins }
    }

    fn check_in(&mut self, id: CustomerId, station_name: String, t: Timestamp) {
        let check_in = Check::new(station_name, t);

        if self.ins.contains_key(&id) {
            return;
        }

        self.ins.insert(id, check_in);
    }

    fn check_out(&mut self, id: CustomerId, station_name: String, t: i32) {
        let check_in = self.ins.remove(&id).expect("check_in not found");

        let trip = Trip::new(t - check_in.t, check_in.station_name.clone(), station_name);
        self.trips.push(trip);
    }

    fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        let trips = search::trips_by_stations(self, start_station, end_station);

        let total_trip_time = trips
            .iter()
            .fold(0, |total_trip_time, t| total_trip_time + t.travel_time);

        let total_trips = trips.iter().count() as f64;
        total_trip_time as f64 / total_trips
    }
}

fn main() {
    case1();
    case2();
}

fn case1() {
    let mut u = UndergroundSystem::new();
    u.check_in(45, "Leyton".to_string(), 3);
    u.check_in(32, "Paradise".to_string(), 8);
    u.check_in(27, "Leyton".to_string(), 10);
    u.check_out(45, "Waterloo".to_string(), 15); // Customer 45 "Leyton" -> "Waterloo" in 15-3 = 12
    u.check_out(27, "Waterloo".to_string(), 20); // Customer 27 "Leyton" -> "Waterloo" in 20-10 = 10
    u.check_out(32, "Cambridge".to_string(), 22); // Customer 32 "Paradise" -> "Cambridge" in 22-8 = 14
    let time1 = u.get_average_time("Paradise".to_string(), "Cambridge".to_string()); // return 14.00000. One trip "Paradise" -> "Cambridge", (14) / 1 = 14

    let time2 = u.get_average_time("Leyton".to_string(), "Waterloo".to_string()); // return 11.00000. Two trips "Leyton" -> "Waterloo", (10 + 12) / 2 = 11
    u.check_in(10, "Leyton".to_string(), 24);
    let time3 = u.get_average_time("Leyton".to_string(), "Waterloo".to_string()); // return 11.00000
    u.check_out(10, "Waterloo".to_string(), 38); // Customer 10 "Leyton" -> "Waterloo" in 38-24 = 14
    let time4 = u.get_average_time("Leyton".to_string(), "Waterloo".to_string());
    // return 12.00000. Three trips "Leyton" -> "Waterloo", (10 + 12 + 14) / 3 = 12

    println!("time1: {time1}");
    println!("time2: {time2}");
    println!("time3: {time3}");
    println!("time4: {time4}");
}

fn case2() {
    let mut u = UndergroundSystem::new();

    u.check_in(10, "Leyton".to_string(), 3);
    u.check_out(10, "Paradise".to_string(), 8); // Customer 10 "Leyton" -> "Paradise" in 8-3 = 5
    let time1 = u.get_average_time("Leyton".to_string(), "Paradise".to_string()); // return 5.00000, (5) / 1 = 5
    u.check_in(5, "Leyton".to_string(), 10);
    u.check_out(5, "Paradise".to_string(), 16); // Customer 5 "Leyton" -> "Paradise" in 16-10 = 6
    let time2 = u.get_average_time("Leyton".to_string(), "Paradise".to_string()); // return 5.50000, (5 + 6) / 2 = 5.5
    u.check_in(2, "Leyton".to_string(), 21);
    u.check_out(2, "Paradise".to_string(), 30); // Customer 2 "Leyton" -> "Paradise" in 30-21 = 9
    let time3 = u.get_average_time("Leyton".to_string(), "Paradise".to_string());
    // return 6.66667, (5 + 6 + 9) / 3 = 6.66667

    println!("time1: {time1}");
    println!("time2: {time2}");
    println!("time3: {time3}");
}
