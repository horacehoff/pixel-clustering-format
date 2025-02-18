use crate::convert;
use std::fs;

pub fn compare() -> (Vec<String>, Vec<usize>, Vec<usize>) {
    let mut name: Vec<String> = Vec::new();
    let mut original: Vec<usize> = Vec::new();
    let mut compressed: Vec<usize> = Vec::new();

    match fs::read_dir("test-images/") {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        if entry.path().file_name().unwrap().to_str().unwrap().contains("credits") {
                            continue;
                        }
                        name.push(entry.path().file_name().unwrap().to_str().unwrap().to_string());
                        original.push(fs::metadata(entry.path()).unwrap().len() as usize);
                        convert(entry.path().to_str().unwrap(), "test.pcf", false);
                        compressed.push(fs::metadata("test.pcf").unwrap().len() as usize);
                        println!("{:?}", entry.path().file_name().unwrap());
                    }
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
    println!("{name:?}");
    println!("{original:?}");
    println!("{compressed:?}");
    return (name, original, compressed);
}