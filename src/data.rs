use crate::encode::convert;
use std::fs;
use std::fs::{DirEntry, File, OpenOptions};
use std::io::{Read, Write};

pub fn compare(args: &[String]) {
    let mut name: Vec<String> = Vec::new();
    let mut original: Vec<usize> = Vec::new();
    let mut compressed: Vec<usize> = Vec::new();
    let mut compressed_lossy: Vec<usize> = Vec::new();

    fn test_entry(
        entry: &DirEntry,
        name: &mut Vec<String>,
        original: &mut Vec<usize>,
        compressed: &mut Vec<usize>,
        compressed_lossy: &mut Vec<usize>,
    ) {
        if entry
            .file_name()
            .to_str()
            .unwrap()
            .to_string()
            .contains("credits")
        {
            return;
        }
        name.push(
            entry
                .path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
        );
        original.push(fs::metadata(entry.path()).unwrap().len() as usize);
        println!("Testing lossless");
        convert(
            entry.path().to_str().unwrap(),
            "test.pcf",
            false,
            false,
            false,
            false,
            false,
        );
        compressed.push(fs::metadata("test.pcf").unwrap().len() as usize);

        let mut various_sizes = Vec::new();
        for b1 in [false, true] {
            for b2 in [false, true] {
                for b3 in [false, true] {
                    for b4 in [false, true] {
                        if b1 || b2 || b3 || b4 {
                            println!("Testing lossy: {b1} {b2} {b3} {b4}");
                            convert(
                                entry.path().to_str().unwrap(),
                                "test.pcf",
                                false,
                                b1,
                                b2,
                                b3,
                                b4,
                            );
                            various_sizes.push(fs::metadata("test.pcf").unwrap().len() as usize);
                        }
                    }
                }
            }
        }
        compressed_lossy.push(*various_sizes.iter().min().unwrap());
        println!("{:?}", entry.path().file_name().unwrap());
    }
    let mut file = File::open("data.txt").unwrap();
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    // if !buffer.is_empty() {
    let result: (Vec<String>, Vec<usize>, Vec<usize>, Vec<usize>) =
        bincode::deserialize(&buffer).unwrap();
    name = result.0;
    original = result.1;
    compressed = result.2;
    compressed_lossy = result.3;
    // }

    let entries = fs::read_dir("test-images/").unwrap();
    for entry in entries {
        let fuck = entry.unwrap();
        if fuck
            .file_name()
            .to_str()
            .unwrap()
            .to_string()
            .starts_with(args.get(1).unwrap())
        {
            test_entry(
                &fuck,
                &mut name,
                &mut original,
                &mut compressed,
                &mut compressed_lossy,
            );
        }
    }
    println!("{name:?}");
    println!("{original:?}");
    println!("{compressed:?}");
    println!("{compressed_lossy:?}");
    let encoded = bincode::serialize(&(name, original, compressed, compressed_lossy)).unwrap();
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("data.txt")
        .unwrap();
    file.write_all(&encoded).unwrap();
}
