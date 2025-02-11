use serialport::SerialPort;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};
use std::time::Duration;
use std::{error::Error, thread};

const PORT_NAME: &str = "/dev/ttyACM0";
const BAUD_RATE: u32 = 9600;

fn main() -> Result<(), Box<dyn Error>> {
    let mut port: Option<Box<dyn SerialPort>> = None;

    let csv_file_path = "data.csv";
    let csv_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(csv_file_path)?;
    let mut csv_writer = csv::Writer::from_writer(csv_file);

    loop {
        match &mut port {
            Some(p) => {
                let mut reader = BufReader::new(&mut **p);
                let mut buffer = String::new();

                match reader.read_line(&mut buffer) {
                    Ok(n) if n > 0 => {
                        let line = buffer.trim();
                        if line.is_empty() {
                            continue;
                        }
                        println!("Received: {}", line);

                        let tokens: Vec<&str> = line.split(',').collect();
                        if tokens.len() >= 2 {
                            let val0 = tokens[0].trim().parse::<f32>().unwrap_or_else(|_| {
                                eprintln!("Failed to parse first value from: {}", line);
                                0.0
                            });
                            let val1 = tokens[1].trim().parse::<f32>().unwrap_or_else(|_| {
                                eprintln!("Failed to parse second value from: {}", line);
                                0.0
                            });

                            if let Err(e) =
                                csv_writer.write_record(&[val0.to_string(), val1.to_string()])
                            {
                                eprintln!("Error writing to CSV: {}", e);
                            }
                            if let Err(e) = csv_writer.flush() {
                                eprintln!("Error flushing CSV writer: {}", e);
                            }
                            println!("Logged: {}, {}", val0, val1);
                        } else {
                            eprintln!("Unexpected line format: {}", line);
                        }
                    }
                    Ok(_) => {
                        thread::sleep(Duration::from_millis(10));
                    }
                    Err(e) => {
                        eprintln!("Error reading from serial port: {}. Resetting port...", e);
                        port = None;
                    }
                }
            }
            None => {
                match serialport::new(PORT_NAME, BAUD_RATE)
                    .timeout(Duration::from_millis(100))
                    .open()
                {
                    Ok(new_port) => {
                        println!("Successfully opened serial port: {}", PORT_NAME);
                        port = Some(new_port);
                    }
                    Err(err) => {
                        println!(
                            "Failed to open serial port: {}. Retrying in 1 second...",
                            err
                        );
                        thread::sleep(Duration::from_secs(1));
                    }
                }
            }
        }
    }
}
