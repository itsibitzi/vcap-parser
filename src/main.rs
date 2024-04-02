use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use fixed_width_data::FixedWidthError;

use crate::abstract_records::process_abstract_record;

mod abstract_records;
mod case_record;
mod fixed_width_data;
mod slice_cursor;
mod types;

fn main() {
    // Just parsing what I presume is the abstract records for now
    let f = File::open("./data/AOCP/CV/PROD/NOBA0001").unwrap();

    let mut total_lines = 0;
    let mut success_lines = 0;

    BufReader::new(f)
        .split(b'\n')
        .enumerate()
        .for_each(|(i, line)| {
            total_lines += 1;
            let line = line.expect("read from disk");

            if !line.is_empty() {
                //println!("{}", String::from_utf8_lossy(&line));

                match process_abstract_record(&line) {
                    Ok(_abstract_record) => {
                        //println!("L{} - {:?}", i, abstract_record);
                        success_lines += 1;
                    }
                    Err(e) => match e {
                        FixedWidthError::RangeTooShort { field_name, range } => {
                            eprintln!(
                                "Line {} too short, could not parse {:?}: {:?}",
                                i, field_name, range
                            )
                        }
                        FixedWidthError::BadNumeric { field_name, range } => {
                            eprintln!(
                                "Line {} bad numeric, could not parse {:?}: {:?}",
                                i, field_name, range
                            )
                        }
                    },
                }
            }
        });

    println!("Sucessful {}/{}", success_lines, total_lines);
}
