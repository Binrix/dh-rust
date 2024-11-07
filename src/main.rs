use std::{fs::File, io::{BufRead, BufReader}, usize};

use serde_json_path::JsonPath;

// #[derive(Default)]
// pub struct PipelineContext<'a> {
//     pub file_name: &'a str,
//     // pub file_content: BufReader<File>
// }

// impl<'a> Default for PipelineContext<'a> {
//     fn default() -> Self {
//         Self {
//             file_name: Default::default(),
//             // file_content: BufReader
//         } 
//     }
// }

/// Reads the content of a file line by line. Replaces sensitive data.
fn read(file_name: &str) -> std::io::Result<()> {
    let reader = BufReader::new(File::open(file_name)?);

    for line_string in reader.lines() {
        match line_string {
            Ok(line) => {
                let json: serde_json::Value = serde_json::from_str(&line).expect("Parsing was not possible");
                let path = JsonPath::parse("$.data.userId").unwrap(); 
                

                let val_of_prop = path.query(&json).exactly_one().unwrap();

                println!("{}", val_of_prop);
            },
            Err(e) => panic!("Error reading the line {}", e)
        }

        // let line = line.expect("Could not get line");

        // let mut iter = lin

        // let start_index = match line.find("userId").map(|i| i + "userId".len() + 1) {
        //     Some(index) => index,
        //     None => return Ok(())
        // };

        // if start_index != 0 {
        //     let end_index = &line[start_index..];


        //     // let word = line.get(start_index..end_index).unwrap();
        //     println!("{}", end_index)
        // }

        // if start_index != None {
        //     // line.replace_range(range, replace_with);
        //     let end_index = line[start_index..];

        // }

        // let new_line = line.replace("userId", "anonymized");

        // println!("line: {}, start index for property: {}", line, start_index);
        // println!("value: {}", line.chars().nth(start_index).unwrap());
    }

    Ok(())
}


pub fn main() {
    let _ = read("example.json");    
}