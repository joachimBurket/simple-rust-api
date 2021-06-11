

use serde::Deserialize;
use csv::ReaderBuilder;
use itertools::Itertools;

/// Measures taken on a specific station at a specific time
#[derive(Debug, Deserialize)]
pub struct MeasuringPoint {
    // Station abbreviation
    #[serde(rename(deserialize = "Station/Location"))]
    station: String,

    // Measure datetime yyyyMMddHHmm
    #[serde(rename(deserialize = "Date"))]
    datetime: String,

    // Air temperature 2m above ground in °C (current value)
    #[serde(rename(deserialize = "tre200s0"))]
    #[serde(deserialize_with = "csv::invalid_option")]
    temp: Option<f32>,

    // Precipitations in mm (10min total)
    #[serde(rename(deserialize = "rre150z0"))]
    #[serde(deserialize_with = "csv::invalid_option")]
    precipitations: Option<f32>,

    // Sunshine duration in min (10min total)
    #[serde(rename(deserialize = "sre000z0"))]
    #[serde(deserialize_with = "csv::invalid_option")]
    sunshine: Option<f32>,
    
    // Global radiation in W/m2 (10min mean)
    #[serde(rename(deserialize = "gre000z0"))]
    #[serde(deserialize_with = "csv::invalid_option")]
    radiation: Option<f32>,
    
    // Relative air humidity 2m above ground (current value)
    #[serde(rename(deserialize = "ure200s0"))]
    #[serde(deserialize_with = "csv::invalid_option")]
    humidity: Option<f32>,
    
    // Dew point 2m above ground (current value)
    #[serde(rename(deserialize = "tde200s0"))]
    #[serde(deserialize_with = "csv::invalid_option")]
    dew_point: Option<f32>,
    
    // Wind direction in ° (10min mean)
    #[serde(rename(deserialize = "dkl010z0"))]
    #[serde(deserialize_with = "csv::invalid_option")]
    wind_direction: Option<f32>,
    
    // Wind speed in km/h (10min mean)
    #[serde(rename(deserialize = "fu3010z0"))]
    #[serde(deserialize_with = "csv::invalid_option")]
    wind_speed: Option<f32>,
    
    // Wind gust (one second) peak in km/h (maximum)
    #[serde(rename(deserialize = "fu3010z1"))]
    #[serde(deserialize_with = "csv::invalid_option")]
    wind_gust_peak: Option<f32>,
    
    // Pressure at station level in hPa (current value)
    #[serde(rename(deserialize = "prestas0"))]
    #[serde(deserialize_with = "csv::invalid_option")]
    pressure: Option<f32>,
    
    // Pressure reduced at see level in hPa (current value)
    #[serde(rename(deserialize = "pp0qffs0"))]
    #[serde(deserialize_with = "csv::invalid_option")]
    pressure_at_see_level: Option<f32>,
}

/// A measuring station information
#[derive(Debug, Deserialize)]
pub struct MeasuringStation {
    // Full name
    #[serde(rename(deserialize = "Station"))]
    name: String,

    // Abbreviation
    #[serde(rename(deserialize = "Abbr."))]
    abbr: String, 

    // Station Type
    #[serde(rename(deserialize = "Station type"))]
    station_type: String,

    // Station Height 
    #[serde(rename(deserialize = "Station height m. a. sea level"))]
    height: u32,

    // Station barometric altitude m. a. ground
    #[serde(rename(deserialize = "Barometric altitude m. a. ground"))]
    #[serde(deserialize_with = "csv::invalid_option")]
    barometric_altitude: Option<u32>,

    // Latitude
    #[serde(rename(deserialize = "Latitude"))]
    latitude: f64,
    
    // Longitude
    #[serde(rename(deserialize = "Longitude"))]
    longitude: f64,

    // Canton abbreviation
    #[serde(rename(deserialize = "Canton"))]
    canton: String,

    // Meaturements (list of measures available)
    #[serde(rename(deserialize = "Measurements"))]
    measurements: String,
}

// Alias to `Box<error::Error>`
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Client to query the MeteoSwiss API
pub struct MeteoSwissApiClient {
    stations_url: String,
    measures_url: String,
}

impl MeteoSwissApiClient {
    const CSV_DELIMITER: u8 = b';';
    const STATION_RESPONSE_TRAILING_LINES: usize = 3;

    // Constructor
    pub fn new(stations_url: String, measures_url: String) -> MeteoSwissApiClient {
        MeteoSwissApiClient {
            stations_url,
            measures_url,
        }
    }

    /// Method removing N trailing lines of a String
    fn remove_trailing_lines(&self, input: String, n: usize) -> String{
        let lines: Vec<String> = input
            .lines()
            .dropping_back(n)
            .map(String::from)
            .collect();
        lines.join("\n")
    }

    /// Returns the network's stations
    pub fn get_stations(&self) -> Result<Vec<MeasuringStation>> {
        let resp = reqwest::blocking::get(&self.stations_url)?;
        let status = resp.status();
        if status.is_success() {
            let txt = resp.text()?;
            let txt = self.remove_trailing_lines(txt, MeteoSwissApiClient::STATION_RESPONSE_TRAILING_LINES);
            
            let mut reader = ReaderBuilder::new().delimiter(MeteoSwissApiClient::CSV_DELIMITER)
                .from_reader(txt.as_bytes());
            
            let mut measuring_stations: Vec<MeasuringStation> = Vec::new();

            for result in reader.deserialize::<MeasuringStation>() {
                let result = result?;
                // println!("{:?}", result);
                measuring_stations.push(result);
            }
            return Ok(measuring_stations);
        }
        println!("Something bad happened: Status = {:?}", status);
        return Err(format!("Something bad happened: Status = {:?}", status))?;
    }

    /// Returns the last measures of the network's stations
    pub fn get_last_measures(&self) -> Result<Vec<MeasuringPoint>> {
        let resp = reqwest::blocking::get(&self.measures_url)?;
        let status = resp.status();
        if status.is_success() {
            let txt = resp.text()?;
            let mut reader = ReaderBuilder::new().delimiter(MeteoSwissApiClient::CSV_DELIMITER)
                .from_reader(txt.as_bytes());
            
            let mut measuring_points: Vec<MeasuringPoint> = Vec::new();

            for result in reader.deserialize::<MeasuringPoint>() {
                let result = result?;
                // println!("{:?}", result);
                measuring_points.push(result);
            }
            return Ok(measuring_points);
        }
        println!("Something bad happened: Status = {:?}", status);
        return Err(format!("Something bad happened: Status = {:?}", status))?;
    }
}