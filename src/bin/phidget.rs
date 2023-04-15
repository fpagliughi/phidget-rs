
use phidget::ReturnCode;

fn main() -> anyhow::Result<()> {
    println!("{}", phidget::library_version()?);
    println!("{}", phidget::library_version_number()?);
    println!("{}", ReturnCode::from(3));
    Ok(())
}
