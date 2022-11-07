use std::env;
use json::{self, JsonValue};
use std::path::Path;
use std::fs::File;
use std::io::Read;
use image::{GenericImageView};
use std::thread;

const EPS: u8 = 5;

#[derive(PartialEq)]
enum ErrorTypes {
    AllGood,
    NoRef(String),                      // <file> was not found
    NoOutput(String),                   // <file> is missing from ref folder
    BadSizes((u32, u32), (u32, u32)),   // Mismatched sizes: (x1, y1) vs (x2, y2)
    BadTypes,                           // The images are not the same type
    // BadFormat,                          // The images have a wrong format
    // NotAnImage,                         // The file is not an image
    EPSError(String, u8, u32, u32)              // Differnce was more than EPS (<difference>) at x, y
}

fn compare_images(path_ref: &String, path_out: &String) -> ErrorTypes {
    if !Path::new(path_ref).exists() {
        return ErrorTypes::NoRef(path_ref.to_string());
    }

    if !Path::new(path_out).exists() {
        return ErrorTypes::NoOutput(path_out.to_string());
    }

	let img_ref = image::open(path_ref).unwrap();
	let img_out = image::open(path_out).unwrap();

	if img_ref.dimensions() != img_out.dimensions() {
		return ErrorTypes::BadSizes(img_ref.dimensions(), img_out.dimensions());
	}

    if img_ref.color() != img_out.color() {
        return ErrorTypes::BadTypes;
    }

    let ref_arr = img_ref.as_bytes();
    let out_arr = img_out.as_bytes();

    for (a, b) in ref_arr.iter().zip(out_arr) {
        if a.abs_diff(*b) > EPS {
            return ErrorTypes::EPSError(path_out.to_string(), a.abs_diff(*b), 100 / img_ref.width(), 100 % img_ref.width());
        }
    }

	ErrorTypes::AllGood
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let test_name = &args[2];

    let mut file = File::open("tests.json").unwrap();
    let mut data: String = String::new();
    file.read_to_string(&mut data).unwrap();

    let json: JsonValue = json::parse(&data).unwrap();

    let mut threads = Vec::new();
    
    for i in 0..json[test_name]["files"].len() {
        let refname = String::from(json[test_name]["files"][i]["ref"].as_str().unwrap().clone());
        let outname = String::from(json[test_name]["files"][i]["output"].as_str().unwrap().clone());
        
        let handle = thread::spawn(move || {
            compare_images(&refname, &outname)
        });

        threads.push(handle);
    }

    let mut error: ErrorTypes = ErrorTypes::AllGood;

    for handle in threads {
        error = handle.join().unwrap();
        if error != ErrorTypes::AllGood {
			break;
		}
    }

    match error {
        ErrorTypes::BadSizes(a, b) => {
            println!("Mismatched sizes: {:?} vs {:?}", a, b);
        },
        ErrorTypes::NoOutput(name) => {
            println!("{name} was not found");
        },
        ErrorTypes::NoRef(name) => {
            println!("{name} ref is missing");
        },
        ErrorTypes::EPSError(name, diff, x, y) => {
            println!("File {name} has a differnce more than {EPS} ({diff}) at {x}, {y}");
        },
        ErrorTypes::BadTypes => {
            println!("The images are not the same type");
        },
        ErrorTypes::AllGood => { 
            println!();
        }
    }
}
