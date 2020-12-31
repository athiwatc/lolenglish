use std::process::Command;
use sysinfo::{ProcessExt, Signal, SystemExt};

use tokio;
use tokio::time::{sleep, Duration};

static LOL_PROCESS_NAME: &'static str = "RiotClientServices.exe";
static LOCALE_PREFIX: &'static str = "--locale=";
static LOCALE_TO_RUN: &'static str = "en_US";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Waiting for LOL to start.");
    let mut system = sysinfo::System::new();

    loop {
        system.refresh_all();
        for proc in system.get_process_by_name(LOL_PROCESS_NAME) {
            let cmd = proc.cmd();
            let cmd_en: Vec<String> = cmd[1..]
                .into_iter()
                .map(|c| {
                    if c.starts_with(LOCALE_PREFIX) {
                        let mut locale_str = LOCALE_PREFIX.to_owned();
                        locale_str.push_str(LOCALE_TO_RUN);
                        locale_str
                    } else {
                        c.to_owned()
                    }
                })
                .collect();

            proc.kill(Signal::Kill);

            println!("{:?}", cmd_en);

            Command::new(&cmd[0])
                .args(cmd_en)
                .spawn()
                .expect("failed to execute process");
            return Ok(());
        }
        sleep(Duration::from_millis(1)).await;
    }
}
