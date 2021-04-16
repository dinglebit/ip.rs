use std::fmt;

use dinglebit_terminal::style;
use reqwest::blocking;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
struct ClientIp {
    ip: String,
    country: Option<String>,
    country_code: Option<String>,
    city: Option<String>,
    continent: Option<String>,
    latitude: Option<f32>,
    longitude: Option<f32>,
    time_zone: Option<String>,
    postal_code: Option<String>,
    org: Option<String>,
    asn: Option<String>,
    subdivision: Option<String>,
    subdivision2: Option<String>,
}

macro_rules! print_if_let {
    ($f:expr, $name:expr, $color:ident, $value:expr) => {
        if let Some(i) = $value {
            write!($f, "{}", style!("{}", $name).cyan())?;
            write!($f, "{}\n", style!("{}", i).$color())?;
        }
    };
}

impl fmt::Display for ClientIp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        print_if_let!(f, "IP:            ", red, Some(&self.ip));
        print_if_let!(f, "Latitude:      ", green, &self.latitude);
        print_if_let!(f, "Longitude:     ", green, &self.longitude);
        print_if_let!(f, "Country Code:  ", green, &self.country_code);
        print_if_let!(f, "Country:       ", white, &self.country);
        print_if_let!(f, "City:          ", white, &self.city);
        print_if_let!(f, "Continent:     ", white, &self.continent);
        print_if_let!(f, "Time Zone:     ", magenta, &self.time_zone);
        print_if_let!(f, "Postal Code:   ", white, &self.postal_code);
        print_if_let!(f, "Organization:  ", white, &self.org);
        print_if_let!(f, "ASN:           ", white, &self.asn);
        print_if_let!(f, "Subdivision:   ", white, &self.subdivision);
        print_if_let!(f, "Subdivision 2: ", white, &self.subdivision2);
        Ok(())
    }
}

fn main() {
    let url = "https://www.iplocate.io/api/lookup/";
    let resp = match blocking::get(url) {
        Ok(resp) => resp,
        Err(e) => {
            eprintln!("getting ip from {}: {}", url, e.to_string());
            std::process::exit(1);
        }
    };

    let resp = match resp.json::<ClientIp>() {
        Ok(resp) => resp,
        Err(e) => {
            eprintln!("deserializing json: {}", e.to_string());
            std::process::exit(1);
        }
    };

    print!("{}", resp);
}
