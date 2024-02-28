use std::fs;
use std::time::Instant;
use std::collections::HashMap;
struct CityResult<'a>{
    name: &'a str,
    min: f64,
    max: f64,
    avg: f64,
}
fn main(){
    let file_path = "weather_stations.csv";
    let start = Instant::now();
    println!("Starting the calculation");
    let mut city_map: HashMap<String, Vec<f64>> = HashMap::new();

    //Parsing Cities and storing data in hashmap of city name as key, and vector of temperature as value;
    for line in fs::read_to_string(file_path).unwrap().lines() {
        let city: String = line.split(";").nth(0).unwrap().to_string();
        let temp: f64 = line.split(";").nth(1).unwrap().parse().unwrap();
        if !city_map.contains_key(&city) {
            city_map.insert(city.clone(), Vec::new());
        }
        city_map.get_mut(&city).unwrap().push(temp);
    }

    //Calculating min max and average for each city
    let mut city_data: Vec<CityResult> = Vec::new();
    city_map.keys().for_each(|f| {
        let v: &Vec<f64> = city_map.get(f).unwrap();
        let mut min = 999.00;
        let mut max = -999.00;
        let mut c = 0.0;
        let mut s = 0.0;
        v.iter().for_each(|t|{
            if *t<min {
                min=*t;
            }
            if *t>max{
                max=*t;
            }
            c+=1.0;
            s+=t;
        });
        let data: CityResult = CityResult{
            name: f,
            min: min,
            max: max,
            avg: s/c,
        };
        city_data.push(data);

    });

    //Sort Cities by name
    city_data.sort_by(|a,b|a.name.to_lowercase().cmp(&b.name.to_lowercase()));


    //Print Cities
    city_data.iter().for_each(|f|{
        println!("city: {} min: {} max: {} avg: {} ",f.name,f.min,f.max,f.avg);
    });
    
    let duration = start.elapsed();

    //print duration in min:sec:ms
    let total_milliseconds = duration.as_millis();
    let milliseconds = total_milliseconds % 1000;
    let total_seconds = total_milliseconds / 1000;
    let seconds = total_seconds % 60;
    let minutes = total_seconds / 60;
    println!("Time elapsed in running the program is: {}:{}:{}",minutes,seconds,milliseconds);
    
}