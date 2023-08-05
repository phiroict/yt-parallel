use std::fs::File;
use std::io::{self, BufRead};
use std::process::{Command, Stdio};
use std::{fs, thread};
use chrono::Local;

fn check_downloader_present() -> bool {
    let command = "yt-dlp";

    let output = Command::new("which")
        .arg(command)
        .output()
        .expect("Failed to execute 'which' command");

    if output.status.success() {
        true
    } else {
        false
    }
}

fn move_to_nas(source: String, target: String) -> bool {
    let output = Command::new("mv").arg(source).arg(target).output().expect("Could not move to the NAS");
    if output.status.success() {
        true
    } else {
        false
    }
}

fn main() -> io::Result<()> {
    let yt_downloader_is_present = check_downloader_present();
    if ! yt_downloader_is_present {
        panic!("yt-dlp is not present, not possible to continue");
    }
    // Create a folder with the current datetime
    let datetime = Local::now();
    let folder_name:String = datetime.format("%Y%m%d").to_string();
    fs::create_dir(&folder_name)?;

    // Open the file
    let file = File::open("videolist.txt")?;

    // Create a vector to store the lines
    let mut lines: Vec<String> = Vec::new();

    // Read the file line by line
    for line in io::BufReader::new(file).lines() {
        // Handle any potential errors
        let line = line?;
        // Add the line to the vector
        lines.push(line);
    }
    // Create an array that can hold all the thread handles so we can join them down the line
    let mut threadpool = vec![];
    // Process the lines in the array, create a thread for each download
    for line in &lines {
        // Do something with each line
        println!("Processing {}", line);
        let cline = line.clone();
        let cfn = String::from(&folder_name);
        let t = thread::spawn(move || {
            // Run yt-dlp process with the line as an argument, by default we remove the
            // sponsor-blocks as they are repetative and most of the time not even relevant
            let _output = Command::new(format!("yt-dlp"))
                .arg("--sponsorblock-remove")
                .arg("default")
                .arg(&cline)
                .current_dir(cfn)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
                .expect("Failed to execute yt-dlp command, you may need to install it.");
        });
        println!("Created thread {:?} for youtube url {}", t.thread().id() ,line);
        threadpool.push(t);
    }

    for t in threadpool {
        println!("Joining thread {:?} ", t.thread().id());
        t.join().expect("Could not join thread");
    }
    // Change your destination path in here.
    println!("Download complete, starting to move to NAS");
    let move_result = move_to_nas(folder_name.clone(), format!("/home/phiro/mounts/Volume_1/youtube/{}", &folder_name));
    if move_result {
        println!("Move complete")
    } else {
        println!("Move directory yourself")
    }
    Ok(())
}
