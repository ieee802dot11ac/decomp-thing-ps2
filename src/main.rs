use std::error::Error;
use std::fs::File;
use std::io::Read;
use yaml_rust2::YamlLoader;

fn main() -> Result<(), Box<dyn Error>> {
    let mut cfg_file_obj = File::open("config/config.yml")?;
    let mut cfg_file_str = String::new();
    cfg_file_obj.read_to_string(&mut cfg_file_str)?;
    let cfg_file_data = YamlLoader::load_from_str(&cfg_file_str)?;
    match cfg_file_data[0].as_str() {
        Some(x) => println!("holy shit this thing is actually {}", x),
        None => println!("fuck"),
    }
    Ok(())
}
