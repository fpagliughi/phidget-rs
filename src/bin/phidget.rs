use phidget::{Phidget, ReturnCode};
use std::{thread, time::Duration};

const TIMEOUT: Duration = Duration::from_millis(5000);

fn main() -> anyhow::Result<()> {
    println!("{}", phidget::library_version()?);
    println!("{}", phidget::library_version_number()?);
    println!("{}", ReturnCode::from(3));

    let mut hum_sensor = phidget::HumiditySensor::new();
    hum_sensor.open_wait(TIMEOUT)?;
    let humidity = hum_sensor.humidity()?;
    println!("Humidity: {}", humidity);

    let mut temp_sensor = phidget::TemperatureSensor::new();
    temp_sensor.open_wait(TIMEOUT)?;
    let temperature = temp_sensor.temperature()?;
    println!("Temperature: {}\n", temperature);

    hum_sensor.set_on_humidity_change_handler(|_s: &phidget::HumiditySensor, humidity: f64| {
        println!("Humidity: {}", humidity);
    })?;

    temp_sensor.set_on_temperature_change_handler(
        |_s: &phidget::TemperatureSensor, temperature: f64| {
            println!("Temerature: {}", temperature);
        },
    )?;

    // ^C handler wakes up the main thread
    ctrlc::set_handler({
        let thr = thread::current();
        move || {
            println!("\nExiting...");
            thr.unpark();
        }
    })
    .expect("Error setting Ctrl-C handler");

    // Block until a ^C wakes us up.
    thread::park();
    Ok(())
}
