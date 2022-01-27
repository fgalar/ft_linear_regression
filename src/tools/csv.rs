use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::SeekFrom;

pub fn get_theta() -> Vec<f64> {
    let filename = "ressources/save.csv";

    let file = File::open(filename)
                        .expect("Can't open save.csv");

    let lines : Vec<_> = BufReader::new(file).lines().collect();
    let thetas : Vec<&str> = lines[1].as_ref()
                                .expect("Bad file format.")
                                .split(',').collect();

    let t0: f64 = thetas[0].parse().expect("Bad character.");
    let t1: f64 = thetas[1].parse().expect("Bad character.");

    vec!(t0, t1)
}

pub fn update_theta(theta: &Vec<f64>) {
    let filename = "ressources/save.csv";
    let file = OpenOptions::new()
                        .write(true)
                        .open(filename)
                        .expect("Unable to open file");
    let mut file = BufWriter::new(file);
    file.seek(SeekFrom::Start(6))
        .expect("Can't put head cursor before first line.");
    let content = format!("{},{}", theta[0].to_string(), theta[1].to_string());
    match file.write(content.as_bytes()) {
        Err(why) => panic!("couldn't write into '{}': {}", filename, why),
        Ok(_) => println!("successfully wrote into '{}'", filename),
    }
}
