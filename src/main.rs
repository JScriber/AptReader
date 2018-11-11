use reader::LogReader;
use model::log::Log;

extern crate regex;

mod reader;
mod model;

fn main() {
    let log_reader: LogReader = LogReader::new();

    let logs: Vec<Log> = log_reader.get_logs();

    for log in logs {
        println!("{} ran {}", log.requester, log.command);

        for installed in log.installed {
            println!("{} version {}", installed.name, installed.version);
        }
    }
}
