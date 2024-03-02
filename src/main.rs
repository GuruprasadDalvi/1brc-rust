use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
struct CityData {
    min: f64,
    max: f64,
    sum: f64,
    count: u32,
}
fn main() {
    let file_path = "weather_stations.csv";
    let separator = ";";
    let file = File::open(file_path);
    let reader = BufReader::new(file.unwrap());
    let start = Instant::now();
    println!("Starting the calculation");
    let mut city_map = HashMap::new();
    let mut city_vec: Vec<String> = Vec::new();

    let lines = reader.lines();

    
    //Parsing Cities and storing data in hashmap of city name as key, and cityData struct as value
    for line in lines {
        let line_val = line.unwrap();

        //Calculating Min an max
        let splitted_line: (&str, &str) = line_val.split_once(separator).unwrap();
        let city: String = splitted_line.0.to_string();
        let temp: f64 = splitted_line.1.parse().unwrap();

        if !city_map.contains_key(&city) {
            city_map.insert(
                city.clone(),
                CityData {
                    min: f64::MAX,
                    max: f64::MIN,
                    sum: 0.0,
                    count: 0,
                },
            );
            city_vec.push(city.clone());
        }
        let current_city = city_map.get_mut(&city).unwrap();
        current_city.min = current_city.min.min(temp);
        current_city.max = current_city.max.max(temp);

        //Storing sum and count for average
        current_city.sum += temp;
        current_city.count += 1;
    }



    city_vec.sort_unstable_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    //Print Cities
    city_vec.iter().for_each(|f| {
        let city_data = city_map.get(f).unwrap();
        println!(
            "city: {} min: {} max: {} avg: {} ",
            f,
            city_data.min,
            city_data.max,
            city_data.sum / (city_data.count as f64)
        );
    });

    let duration = start.elapsed();

    //print duration in min:sec:ms
    let total_milliseconds = duration.as_millis();
    let milliseconds = total_milliseconds % 1000;
    let total_seconds = total_milliseconds / 1000;
    let seconds = total_seconds % 60;
    let minutes = total_seconds / 60;
    println!(
        "Time elapsed in running the program is: {}:{}:{}",
        minutes, seconds, milliseconds
    );
}
