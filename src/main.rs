use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;
use std::iter::Iterator;
use std::time::Duration;

#[derive(Deserialize, Serialize, Debug)]
struct Feature {
    #[serde(rename = "type")]
    feature_type: String,
    geometry: Value,
    properties: Properties,
}

#[derive(Deserialize, Serialize, Debug)]
struct Properties {
    meta: Value,
    timeseries: Vec<Timeseries>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Timeseries {
    time: String,
    data: Data,
}

#[derive(Deserialize, Serialize, Debug)]
struct Data {
    instant: Value,
    next_12_hours: Option<Value>,
    next_1_hours: Option<Next1Hours>,
    next_6_hours: Option<Value>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Next1Hours {
    summary: Summary,
    details: Value,
}

#[derive(Deserialize, Serialize, Debug)]
struct Summary {
    #[serde(rename = "symbol_code")]
    symbol_code: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // API GET
    let client = reqwest::Client::new();
    let res = client
        .get("https://api.met.no/weatherapi/locationforecast/2.0/compact?lat=60.12237&lon=11.47005")
        .header("Accept", "text/plain")
        .header("User-Agent", "YOUR_UNIQUE_STRING_HERE") // The API requires a user-agent be supplied. Anything you want, just make it unique.
        .timeout(Duration::from_secs(3))
        .send()
        .await
        .map_err(|_| "Failed".to_owned())? // Map any error message to a new error message containing just the word 'Failed'
        .text()
        .await
        .map_err(|_| "Failed".to_owned())?; // Map any error message to a new error message containing just the word 'Failed'

    let data = r#res;
    let feature: Feature = serde_json::from_str(&data).unwrap();
    let symbol_codes: Vec<String> = feature
        .properties
        .timeseries
        .iter()
        .take(6) // Collect only the first 6 hours
        .filter_map(|timeseries| {
            // Filter out any entries without the required property `.next_1_hours`
            timeseries
                .data
                .next_1_hours
                .as_ref()
                .map(|next_1_hours| next_1_hours.summary.symbol_code.clone())
                .or_else(|| Some("".to_string()))
        })
        .collect();

    // Concatenate the six symbols into one string
    let mut symbols: String = " ".to_owned();
    for symbol_code in symbol_codes {
        symbols.push_str(symbol_icon(&symbol_code));
    }

    println!("{}", symbols);
    Ok(())
}

// Function to replace the collected weather data with icons.
// This uses the font 'Weather Icons' (https://erikflowers.github.io/weather-icons/) in my case,
// but you can substitute any icon set you desire by changing these entries.
fn symbol_icon(value: &str) -> &str {
    // Sometimes the data is suffixed by e.g. '_day'. Let's remove that part.
    let (first_part, _) = value.split_at(value.find('_').unwrap_or(value.len()));

    match first_part {
        "clearsky" => " ",
        "cloudy" => " ",
        "fair" => " ",
        "fog" => " ",
        "heavyrain" => " ",
        "heavyrainandthunder" => " ",
        "heavyrainshowers" => " ",
        "heavyrainshowersandthunder" => " ",
        "heavysleet" => " ",
        "heavysleetandthunder" => " ",
        "heavysleetshowers" => " ",
        "heavysleetshowersandthunder" => " ",
        "heavysnow" => " ",
        "heavysnowandthunder" => " ",
        "heavysnowshowers" => " ",
        "heavysnowshowersandthunder" => " ",
        "lightrain" => " ",
        "lightrainandthunder" => " ",
        "lightrainshowers" => " ",
        "lightrainshowersandthunder" => " ",
        "lightsleet" => " ",
        "lightsleetandthunder" => " ",
        "lightsleetshowers" => " ",
        "lightsnow" => " ",
        "lightsnowandthunder" => " ",
        "lightsnowshowers" => " ",
        "lightssleetshowersandthunder" => " ",
        "lightssnowshowersandthunder" => " ",
        "partlycloudy" => " ",
        "rain" => " ",
        "rainandthunder" => " ",
        "rainshowers" => " ",
        "rainshowersandthunder" => " ",
        "sleet" => " ",
        "sleetandthunder" => " ",
        "sleetshowers" => " ",
        "sleetshowersandthunder" => " ",
        "snow" => " ",
        "snowandthunder" => " ",
        "snowshowers" => " ",
        "snowshowersandthunder" => " ",
        _ => " ",
    }
}
