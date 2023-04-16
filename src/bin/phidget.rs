
use std::time::Duration;
use phidget::{Phidget, ReturnCode};

const TIMEOUT: Duration = Duration::from_millis(5000);

fn main() -> anyhow::Result<()> {
    println!("{}", phidget::library_version()?);
    println!("{}", phidget::library_version_number()?);
    println!("{}", ReturnCode::from(3));

    let mut sensor = phidget::HumiditySensor::new();
    sensor.open_wait(TIMEOUT)?;
    let humidity = sensor.humidity()?;
    println!("Humidity: {}\n", humidity);

    sensor.set_on_humidity_change_handler(|_s: &phidget::HumiditySensor, humidity: f64| {
        println!("Humidity: {}", humidity);
    })?;

    loop {
        std::thread::sleep(Duration::from_millis(250));
    }

    //Ok(())
}
