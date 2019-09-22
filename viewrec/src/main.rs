#[macro_use]
extern crate clap;
extern crate rex;

use clap::{App, Arg};
use rex::build_format;
use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("data")
                .short("d")
                .long("data")
                .value_name("DATA")
                .required(true)
                .help("Path to the data file you want to view")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("format")
                .short("f")
                .long("format")
                .value_name("FORMAT")
                .required(true)
                .help("Path to the format file that describes your data file")
                .takes_value(true),
        )
        .get_matches();

    let data_path: String = matches.value_of("data").map(|s| s.to_owned()).unwrap();
    let format_path: String = matches.value_of("format").map(|s| s.to_owned()).unwrap();

    let format_file = File::open(format_path).unwrap();
    let format_lines = BufReader::new(format_file)
        .lines()
        .map(|l| l.unwrap())
        .collect();
    let format_fields = build_format(format_lines);

    let longest_field_name_len = format_fields.iter().fold(0, |acc, f| max(acc, f.0.len()));
    let total_field_size = format_fields.iter().fold(0, |acc, f| acc + f.1);
    let data_file = File::open(data_path).unwrap();
    let mut line_counter = 0;
    for line in BufReader::new(data_file).lines() {
        println!("Record: {}", line_counter);
        if let Ok(l) = line {
            let line_bytes = l.into_bytes();
            let mut field_counter: usize = 0;
            for field in &format_fields {
                let (name, width) = field;
                println!(
                    "{:fw$} => {:?}",
                    name,
                    String::from_utf8(line_bytes[field_counter..(field_counter + *width)].to_vec())
                        .unwrap(),
                    fw = &longest_field_name_len
                );
                field_counter += width;
            }
        }
        line_counter += 1
    }

    println!(
        "Dumped {} records {} bytes wide.",
        line_counter, total_field_size
    );
}
