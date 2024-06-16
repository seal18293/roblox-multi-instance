use std::{
    env::args,
    os::windows::process::CommandExt,
    process::{Command, Stdio},
    thread,
    time::Duration,
};

use interprocess::local_socket::{
    tokio::Stream,
    traits::tokio::{Listener, Stream as StreamTrait},
    GenericNamespaced, ListenerOptions, ToNsName,
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

extern "C" {
    fn create_mutex();
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = args().collect();
    if Command::new("tasklist")
        .args(&["/fi", "imagename eq RobloxPlayerBeta.exe"])
        .stdout(Stdio::piped())
        .output()
        .unwrap()
        .stdout
        .starts_with(b"\r\nImage Name")
    {
        println!("ROBLOX IS CURRENTLY RUNNING! Closing roblox is recommended before starting/stopping. Stopping while roblox is running will close all but 1 windows.")
    }
    if args.len() == 1 {
        let stream = Stream::connect(
            "seal.roblox.multi-instance/main"
                .to_ns_name::<GenericNamespaced>()
                .unwrap(),
        )
        .await;
        if stream.is_err() {
            if dialoguer::Confirm::new()
                .with_prompt("Start?")
                .interact()
                .unwrap()
            {
                println!("Starting");
                let listener = ListenerOptions::new()
                    .name(
                        "seal.roblox.multi-instance/start"
                            .to_ns_name::<GenericNamespaced>()
                            .unwrap(),
                    )
                    .create_tokio()
                    .unwrap();
                Command::new(&args[0])
                    .arg("start")
                    .stdin(Stdio::null())
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .creation_flags(/*DETACHED_PROCESS*/ 0x00000008)
                    .spawn()
                    .unwrap();
                listener.accept().await.unwrap();
                println!("Started. Exiting in 5 seconds.");
                thread::sleep(Duration::from_secs(5));
            }
        } else {
            if dialoguer::Confirm::new()
                .with_prompt("Stop?")
                .interact()
                .unwrap()
            {
                let mut stream = stream.unwrap();
                stream.write(&[1]).await.unwrap();
            }
        }
    } else if args.len() == 2 && args[1].as_str() == "start" {
        let stream = Stream::connect(
            "seal.roblox.multi-instance/start"
                .to_ns_name::<GenericNamespaced>()
                .unwrap(),
        )
        .await;
        if stream.is_ok() {
            let listener = ListenerOptions::new()
                .name(
                    "seal.roblox.multi-instance/main"
                        .to_ns_name::<GenericNamespaced>()
                        .unwrap(),
                )
                .create_tokio()
                .unwrap();
            unsafe {
                create_mutex();
            }
            loop {
                let mut conn = listener.accept().await.unwrap();
                let mut buf = [0_u8; 1];
                if conn.read(&mut buf).await.unwrap() > 0 {
                    match buf[0] {
                        1 => break,
                        _ => {}
                    }
                };
            }
        } else {
            eprintln!("DO NOT START DIRECTLY WITH THE start ARGUMENT!");
        }
    }
}
