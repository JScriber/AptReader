use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
extern crate regex;

use regex::Regex;
use regex::{ Captures, Error };

pub struct Package {
    name: String,
    version: String
}

pub struct Log {
    date: String,
    time: String,
    command: String,
    requester: String,
    automatic_action: bool,
    installed: Vec<Package>,
    removed: Vec<Package>,
    purged: Vec<Package>,
    upgraded: Vec<Package>
}


pub struct LogReader {}

impl LogReader {
    // Constructor.
    pub fn new() -> LogReader {
        LogReader {}
    }

    // Loads all the logs.
    pub fn get_logs(&self) -> Vec<Log> {
        let mut logs: Vec<Log> = self.gen_logs();
        logs.drain(0..1);

        logs
    }

    fn gen_logs(&self) -> Vec<Log> {
        let text_logs: Vec<String> = self.text_logs();
        let mut logs: Vec<Log> = Vec::new();

        for text_log in text_logs {
            let mut regex: Regex;
            let mut captures: Option<Captures>;

            let mut date: String = String::new();
            let mut time: String = String::new();
            let mut command: String = String::new();

            // Get the date.
            regex = Regex::new(r"(?m)(\d{4}\-\d{2}\-\d{2})  (\d{2}:\d{2}:\d{2})").unwrap();
            captures = regex.captures(&text_log);

            match captures {
                Some(capture) => {
                    date = capture[1].to_string();
                    time = capture[2].to_string();
                },
                None => {}
            }

            // Get the command.
            regex = Regex::new(r"Commandline: (.+)").unwrap();
            captures = regex.captures(&text_log);

            match captures {
                Some(capture) => command = capture[1].to_string(),
                None => {}
            }

            logs.push(Log {
                date, time, command,
                requester: String::new(),
                automatic_action: false,
                installed: Vec::new(),
                removed: Vec::new(),
                purged: Vec::new(),
                upgraded: Vec::new()
            });
        }

        logs
    }


    // Return a list of each text log.
    fn text_logs(&self) -> Vec<String> {
        let content = self.read_logs().unwrap();

        let mut logs: Vec<String> = Vec::new();
        let mut buffer: String = String::new();

        for line in content.split('\n') {

            if line.len() == 0 {
                logs.push(buffer);
                buffer = String::new();
            } else {
                buffer.push_str(&line);
                buffer.push('\n');
            }
        }

        logs
    }

    // Retrieve all the logs.
    fn read_logs(&self) -> std::io::Result<String> {
        let file: File = File::open("/var/log/apt/history.log").unwrap();

        let mut buf_reader: BufReader<File> = BufReader::new(file);

        let mut content: String = String::new();

        buf_reader.read_to_string(&mut content)?;

        Ok(content)
    }
}

fn main() {
    let log_reader: LogReader = LogReader::new();

    let logs: Vec<Log> = log_reader.get_logs();

    for log in logs {
        println!("Date: {}, Time: {}, Command: {}", log.date, log.time, log.command);
    }
}
