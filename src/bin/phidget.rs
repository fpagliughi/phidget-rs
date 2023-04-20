use phidget::Phidget;
use std::{thread, time::Duration};

const TIMEOUT: Duration = Duration::from_millis(5000);

fn main() -> anyhow::Result<()> {
    println!("{}", phidget::library_version()?);
    println!("{}", phidget::library_version_number()?);

    let mut hum_sensor = phidget::HumiditySensor::new();
    phidget::phidget::set_on_attach_handler(&mut hum_sensor, |_| {
        println!("Humidity sensor attached");
    })?;
    phidget::phidget::set_on_detach_handler(&mut hum_sensor, |_| {
        println!("Humidity sensor detached");
    })?;
    hum_sensor.open_wait(TIMEOUT)?;
    println!(
        "Humidity Device Class: [{:?}, {}] {}",
        hum_sensor.device_class()?,
        hum_sensor.device_class()? as u32,
        hum_sensor.device_class_name()?
    );
    println!(
        "Humidity Channel Class: [{:?}, {}] {}",
        hum_sensor.channel_class()?,
        hum_sensor.channel_class()? as u32,
        hum_sensor.channel_class_name()?
    );
    let humidity = hum_sensor.humidity()?;
    println!("Humidity: {}", humidity);

    let mut temp_sensor = phidget::TemperatureSensor::new();
    phidget::phidget::set_on_attach_handler(&mut temp_sensor, |_| {
        println!("Temperature sensor attached");
    })?;
    phidget::phidget::set_on_detach_handler(&mut temp_sensor, |_| {
        println!("Temperature sensor detached");
    })?;
    temp_sensor.open_wait(TIMEOUT)?;
    println!(
        "Temperature Device Class: [{:?}, {}] {}",
        temp_sensor.device_class()?,
        temp_sensor.device_class()? as u32,
        temp_sensor.device_class_name()?
    );
    println!(
        "Temperature Channel Class: [{:?}, {}] {}",
        temp_sensor.channel_class()?,
        temp_sensor.channel_class()? as u32,
        temp_sensor.channel_class_name()?
    );
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
