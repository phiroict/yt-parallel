use std::fs::File;
use std::io::{self, BufRead};
use std::process::{Command, Stdio};
use std::{env, fs, thread};
use chrono::Local;
use std::sync::mpsc::sync_channel;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Location of the videolist.txt file
    #[arg(short, long, default_value_t = String::from("./videolist.txt"))]
    location_video_list: String,
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

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
    // Clean up failed downloads
    let _ = Command::new("rm")
        .arg("-f")
        .arg(format!("{source}/{target}/{}", "*.part"))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Could not delete the part remainders");
    // Move over to the shared folders for serving
    let output = Command::new("mv")
        .arg(source)
        .arg(target)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Could not move to the NAS");
    // Print out errors and other feedback of the move
    println!("StOut: {:?}", String::from_utf8(output.stdout).unwrap());
    println!("StErr: {:?}", String::from_utf8(output.stderr).unwrap());
    if output.status.success() {
        true
    } else {
        false
    }
}


fn main() -> io::Result<()> {
    // Get arguments commandline
    let args = Args::parse();
    println!("File to parse: {}", args.location_video_list);
    // Get the current version, this is baked into the application and can be extracted as a ENV var

    println!("Running version {}", VERSION);
    let yt_downloader_is_present = check_downloader_present();
    if !yt_downloader_is_present {
        panic!("yt-dlp is not present, not possible to continue");
    }
    // Create a folder with the current datetime
    let datetime = Local::now();
    let folder_name:String = datetime.format("%Y%m%d").to_string();
    fs::create_dir(&folder_name)?;

    // Open the file, hardcoded here as it is part of te fixed setup.
    let file = File::open(args.location_video_list)?;

    // Create a vector to store the lines that consists of urls to a youtube (or other) clip.
    let mut lines: Vec<String> = Vec::new();

    // Read the file line by line
    for line in io::BufReader::new(file).lines() {
        // Handle any potential errors, we fail the whole process here as I do not expect failed entries
        let line = line?;
        // Add the line to the vector so we can feed it to yt-dlp
        lines.push(line);
    }
    // Some information we want to keep track of to tell the user where we are in the process.
    let number_of_items = lines.len();
    let mut iterator_items_index = 1;
    // Setup the communication with the threads, we create the number of download channels.
    let (tx, rx) = sync_channel(lines.len());
    // Create an array that can hold all the thread handles so we can join them down the line
    let mut thread_pool = vec![];
    // Process the lines in the array, create a thread for each download
    for line in &lines {
        let tx= tx.clone();
        // Do something with each line
        println!("Processing {}", line);
        // We move the line into the thread, so we cannot use it anymore after that, so we make a
        // clone that enables us to grab it later. Same story for the folder name. We use it for all
        // the threads so we cannot move the original. The String::from is also a way of cloning the
        // string but also casting it from &str to String.
        let cline = line.clone();
        let cfn = String::from(&folder_name);
        let t = thread::spawn(move || {
            // Run yt-dlp process with the line as an argument, by default we remove the
            // sponsor-blocks as they are repetitive and most of the time not even relevant
            // I am not using the output in normal use, so I prepend it with a "_". For debugging
            // it can be useful. The in and output are buffered
            let _output = Command::new(format!("yt-dlp"))
                .arg("--sponsorblock-remove")
                .arg("default")
                .arg("--retries")
                .arg("infinite")
                .arg("--fragment-retries")
                .arg("infinite")
                .arg("--buffer-size")
                .arg("16K")
                .arg(&cline)
                .current_dir(cfn)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
                .expect("Failed to execute yt-dlp command, you may need to (re)install it. \
                Or make sure it is in PATH of this executable");
            tx.send(format!("Downloaded {}", &cline)).expect("Could not sent message");
        });
        println!("Created thread {:?} for youtube url {line}", t.thread().id());
        thread_pool.push(t);
    }
    // Need to drop the transmitter as otherwise the receiver never stops listening.
    // It will keep tx alive to process the outstanding receivers, if these are all sent,
    // the channels are dropped and the while loop below will exit.
    drop(tx);
    // While there a channels open, wait till they all have sent their message, then when there are
    // none left, the recv will fail (We dropped the transmitter above) and we leave the loop.
    while let Ok(msg) = rx.recv() {
        println!("{msg}, {iterator_items_index} from {number_of_items}");
        iterator_items_index += 1;
    }
    // Join all threads to we can start moving when all downloads have been completed.
    for t in thread_pool {
        t.join().expect("Could not join thread");
    }
    // Change your destination path in here, as I am using two OSes I make a selection here how to
    // handle the move, pretty sure this will not scale to your setup 😉.

    // Rust has some useful constants baked in, one of them is the OS that holds the OS it is running on.
    let os_running = env::consts::OS;
    println!("Download complete, starting to move to NAS, according to OS: {os_running}" );
    // Default when running linux, I run Arch by the way 😎
    let mut path_to_nas = "/home/phiro/mounts/Volume_1/youtube/";
    if os_running.eq("macos") {
        path_to_nas = "/Volumes/huge/media/youtube/";
    }
    // Using the MacOS/Linux move tool here, there are ways to do this in Rust but it is a bit
    // cumbersome and I did not feel like reinventing the mv statement.
    let move_result = move_to_nas(folder_name.clone(), format!( "{}{}", path_to_nas, &folder_name));
    if move_result {
        println!("Move complete")
    } else {
        println!("Move not possible now, move directory yourself")
    }
    Ok(())
}
