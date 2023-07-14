extern crate tokio;
extern crate geolocation;
extern crate reqwest;
use serde_json::{Value};
use chrono;
use std::env;

#[tokio::main]
async fn main() {
    let ip: String = get_ip_adress().await;
    let city: String  = get_city(&ip).await;
    let city: String = city[1..city.len()-1].to_string();
    let weather = get_weather(&city).await;
    output(weather, city);
}


async fn get_ip_adress() -> String{
    let ip: String = reqwest::get("https://ipinfo.io/ip").await.unwrap().text().await.unwrap();
    return ip;
}


async fn get_city(ip: &String) -> String{
    geolocation::find(&ip).unwrap().city.to_string()
}


async fn get_weather(city: &String) -> Value{
    let api = env::var("OWM_API").unwrap().to_string();
    let url = format!("https://api.openweathermap.org/data/2.5/weather?q={}&appid={}", city, api);
    let weather_data: String = reqwest::get(url).await.unwrap().text().await.unwrap();
    return serde_json::from_str(&weather_data).unwrap();
}


fn output(weather: Value, city: String){
    let mut temp: f64 = weather["main"]["temp"].to_string().parse().unwrap();
    temp += -273.15;
    let time = chrono::offset::Utc::now();
    print!("{}[2J", 27 as char);
    println!("City: {city}");
    println!("Temperature: {:.2}°C", temp);
    println!("Time: {}", time);
}