use crate::convert;
use std::fs;

pub fn compare() -> (Vec<String>, Vec<usize>, Vec<usize>) {
    let mut name: Vec<String> = Vec::new();
    let mut original: Vec<usize> = Vec::new();
    let mut compressed: Vec<usize> = Vec::new();
    let mut compressed_lossy: Vec<usize> = Vec::new();

    let entries = fs::read_dir("test-images/").unwrap();
    for entry in entries {
        let entry = entry.unwrap();
        if entry.path().file_name().unwrap().to_str().unwrap().contains("credits") {
            continue;
        }
        name.push(entry.path().file_name().unwrap().to_str().unwrap().to_string());
        original.push(fs::metadata(entry.path()).unwrap().len() as usize);
        println!("Testing lossless");
        convert(entry.path().to_str().unwrap(), "test.pcf", false, false,false,false,false,false);
        compressed.push(fs::metadata("test.pcf").unwrap().len() as usize);


        let mut various_sizes = Vec::new();
        for b1 in [false, true] {
            for b2 in [false, true] {
                for b3 in [false, true] {
                    for b4 in [false, true] {
                        if b1 || b2 || b3 || b4 {
                            println!("Testing lossy: {b1} {b2} {b3} {b4}");
                            convert(entry.path().to_str().unwrap(), "test.pcf", false, true, b1, b2, b3, b4);
                            various_sizes.push(fs::metadata("test.pcf").unwrap().len() as usize);
                        }
                    }
                }
            }
        }
        compressed_lossy.push(*various_sizes.iter().min().unwrap());
        println!("{:?}", entry.path().file_name().unwrap());
    }
    println!("{name:?}");
    println!("{original:?}");
    println!("{compressed:?}");
    println!("{compressed_lossy:?}");
    (name, original, compressed)
}