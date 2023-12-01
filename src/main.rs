mod logging;
mod tests;

use crate::logging::initialize_logging;
use chrono::Local;
use clap::Parser;
use log::{debug, error, info, trace, warn};
use std::fs::File;
use std::io::{self, BufRead};
use std::process::{exit, Command, Stdio};
use std::sync::mpsc::sync_channel;
use std::{env, fs, thread};
use which::which;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Location of the videolist.txt file
    #[arg(short, long, default_value_t = String::from("./videolist.txt"))]
    location_video_list: String,
    #[arg(short, long, default_value_t = String::from("yt-dlp"))]
    video_download_tool: String,
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Check downloader present,
/// Checks whether the yt-dlp app is attainable in the path on the OS.
/// # Parameters
/// command [String]: The command that is the actual appication to download the yt file with.
/// # Returns
/// True when present, else false. Does not processes errors, it will return false on error.
pub fn check_downloader_present(command: String) -> bool {
    let is_present = which(command);
    match is_present {
        Ok(_) => true,
        Err(_) => false,
    }
}

/// Moves the downloaded yt videos to the target, usually a NAS or a shared folder.
/// It will delete *.part files, then move the folder.
/// # Parameters
/// source - string of the path pointing the source, the path must exist on the system and accessible for read/write
/// target - string of the destination, the path must exist on the system and accessible for read/write
/// # Returns
/// True on succesful move, else false, it will panic out when a system error occurs.
fn move_to_nas(source: String, target: String) -> bool {
    debug!("Entered the move_nas function");
    let os_running = env::consts::OS;
    debug!("OS is {os_running} branche");
    if os_running.eq("windows") {
        info!("Download complete, starting to move to NAS, according to OS: {os_running}");
        info!("Remove the partial failed downloads from {source}");
        let target_wildcard = [target.clone(), "\\*".to_string()].join("");
        debug!("About to run the del command deleting the partial files left `cmd /C DEL {source}*.part`");
        let _ = Command::new("cmd")
            .arg("/C")
            .arg("DEL")
            .arg(format!("{source}/{}", "*.part"))
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .expect("Could not delete the part remainders");
        // Move over to the shared folders for serving
        info!("Do the actual move from {source} to {target_wildcard} in windows this is an XCOPY with a delete source dir at the end");
        //Create the target dir as it is not created by the xcopy
        debug!("Creating target directory {}", target.clone());
        let result = fs::create_dir(target.clone());
        match result {
            Ok(_) => {
                info!("Created target folder")
            }
            Err(e) => {
                warn!("The folder creation failed, will attempt to continue {e}");
            }
        }
        let output = Command::new("cmd")
            .arg("/C")
            .arg("XCOPY")
            .arg(source.clone())
            .arg(target_wildcard.clone())
            .arg("/e")
            .arg("/q")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .expect("Could not copy to the NAS");
        fs::remove_dir_all(source.clone())
            .expect("Could not delete the source files, delete them yourself.");
        // Print out errors and other feedback of the move
        info!("StOut: {:?}", String::from_utf8(output.stdout).unwrap());
        info!("StErr: {:?}", String::from_utf8(output.stderr).unwrap());
        if output.status.success() {
            true
        } else {
            false
        }
    } else {
        // Clean up failed downloads
        info!("Download complete, starting to move to NAS, according to OS: {os_running}");
        info!("Remove the partial failed downloads");
        let _ = Command::new("rm")
            .arg("-f")
            .arg(format!("{target}/{}", "*.part"))
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .expect("Could not delete the part remainders");
        // Move over to the shared folders for serving
        info!("Do the actual move");
        let output = Command::new("mv")
            .arg(source)
            .arg(target)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .expect("Could not move to the NAS");
        // Print out errors and other feedback of the move
        info!("StOut: {:?}", String::from_utf8(output.stdout).unwrap());
        info!("StErr: {:?}", String::from_utf8(output.stderr).unwrap());
        if output.status.success() {
            true
        } else {
            false
        }
    }
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    initialize_logging(args.debug_level);
    let time_start = Local::now();
    let start_time = time_start.format("%Y-%m-%dT%H:%M:%S");
    info!("Starting the process at {start_time}");
    // Get arguments commandline

    info!("File to parse: {}", args.location_video_list);
    info!("Downloadtool to use: {}", args.video_download_tool);
    // Get the current version, this is baked into the application and can be extracted as a ENV var

    info!("Running version {}", VERSION);
    let yt_downloader_is_present = check_downloader_present(args.video_download_tool);
    if !yt_downloader_is_present {
        error!("yt-dlp is not present, not possible to continue");
        exit(0x0002);
    }
    // Create a folder with the current datetime
    debug!("Starting creating source folder");
    let datetime = Local::now();
    let folder_name: String = datetime.format("%Y%m%d").to_string();
    debug!("About to create folder {}", &folder_name);
    let create_folder_result = fs::create_dir(&folder_name);
    match create_folder_result {
        Ok(_) => {debug!("Source folder created")}
        Err(e) => {error!("Could not create the folder, it may already exist: {:?}", e)}
    }

    // Open the file, gets the name from the params or it takes the default.
    debug!(
        "Opening the video location file at {}",
        args.location_video_list
    );
    let file = File::open(args.location_video_list.clone());
    match file {
        Ok(fs) => {
            info!("File found and opened");
            let process_result = process_videos(&folder_name, fs);
            match process_result {
                Ok(_) => {
                    info!("Processing video completed")
                }
                Err(e) => {
                    error!("Processing videos encountered an error: {:?}", e);
                    exit(0x0003)
                }
            }
        }
        Err(e) => {
            error!(
                "Could not find the videolist at {} for reason {:?}, leaving",
                args.location_video_list, e
            );
            exit(0x0001);
        }
    }

    trace!("Getting the end time");
    let time_end = Local::now();
    let end_time = time_end.format("%Y-%m-%dT%H:%M:%S");
    let time_passed = time_end - time_start;

    info!(
        "Process concluded at {end_time} while started at {start_time} it took {} seconds",
        time_passed.to_string()
    );
    Ok(())
}

fn process_videos(folder_name: &String, file: File) -> Result<(), Box<dyn std::error::Error>> {
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
        let tx = tx.clone();
        // Do something with each line
        info!("Processing {}", line);
        // We move the line into the thread, so we cannot use it anymore after that, so we make a
        // clone that enables us to grab it later. Same story for the folder name. We use it for all
        // the threads so we cannot move the original. The String::from is also a way of cloning the
        // string but also casting it from &str to String.
        let cline = line.clone();
        let cfn = String::from(folder_name);
        let t = thread::spawn(move || {
            // Run yt-dlp process with the line as an argument, by default we remove the
            // sponsor-blocks as they are repetitive and most of the time not even relevant
            // I am not using the output in normal use, so I prepend it with a "_". For debugging
            // it can be useful. The in and output are buffered
            debug!(
                "I am in thread {:?} starting downloading {}",
                thread::current().id(),
                &cline
            );
            let output = Command::new(format!("yt-dlp"))
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
                .expect(
                    "Failed to execute yt-dlp command, you may need to (re)install it. \
                Or make sure it is in PATH of this executable",
                );
            debug!(
                "I am in thread {:?} completed downloading {}",
                thread::current().id(),
                &cline
            );
            debug!(
                "Thread {:?} StOut: {:?}",
                thread::current().id(),
                String::from_utf8(output.stdout).unwrap()
            );
            debug!(
                "Thread {:?} StErr: {:?}",
                thread::current().id(),
                String::from_utf8(output.stderr).unwrap()
            );
            trace!(
                "About the sent message to main thread from thread {:?}",
                thread::current().id()
            );
            tx.send(format!("Downloaded {}", &cline))
                .expect("Could not sent message");
            trace!(
                "Message to main thread from thread {:?} sent",
                thread::current().id()
            );
        });
        // End thread creation.
        info!(
            "Created thread {:?} for youtube url {line}",
            t.thread().id()
        );
        thread_pool.push(t);
    }
    // Need to drop the transmitter as otherwise the receiver never stops listening.
    // It will keep tx alive to process the outstanding receivers, if these are all sent,
    // the channels are dropped and the while loop below will exit.
    drop(tx);
    // While there a channels open, wait till they all have sent their message, then when there are
    // none left, the recv will fail (We dropped the transmitter above) and we leave the loop.
    while let Ok(msg) = rx.recv() {
        info!("{msg}, {iterator_items_index} from {number_of_items}");
        iterator_items_index += 1;
    }
    // Join all threads to we can start moving when all downloads have been completed.
    for t in thread_pool {
        let current_thread = t.thread().id().clone();
        trace!("About the join thread {:?}", t.thread().id().clone());
        let join_result = t.join(); //After join the t variable is moved and cannot be referenced again.
        match join_result {
            Ok(jr) => {
                debug!("Join of thread {:?} has succeeded", jr);
            }
            Err(e) => {
                error!("Could not join thread {:?} thread is terminated and consider download lost", e);

            }
        }

        trace!("Joined thread {:?}", current_thread);
    }
    // Change your destination path in here, as I am using two OSes I make a selection here how to
    // handle the move, pretty sure this will not scale to your setup 😉.

    // Rust has some useful constants baked in, one of them is the OS that holds the OS it is running on.
    let os_running = env::consts::OS;

    // Default when running linux, I run Arch by the way 😎
    debug!("Set the path to linux path as default, other OS will overwrite, the current OS is {os_running}");
    let mut path_to_nas = "/home/phiro/";
    let windows_path = ["M:\\youtube", "\\"].join("");
    if os_running.eq("macos") {
        debug!("Set the path as set in macos overwriting the linux path");
        path_to_nas = "/Volumes/huge/media/youtube/";
    } else if os_running.eq("windows") {
        debug!("Set the path as set in windows overwriting the linux path");
        path_to_nas = &windows_path;
    }
    // Using the MacOS/Linux move tool here, there are ways to do this in Rust but it is a bit
    // cumbersome and I did not feel like reinventing the mv statement.
    debug!("Going into the move result function");
    let move_time_start = Local::now();
    let move_result = move_to_nas(
        folder_name.clone(),
        format!("{}{}", path_to_nas, &folder_name),
    );
    trace!("Evaluating result move {:?}", move_result);
    if move_result {
        info!("Move complete")
    } else {
        warn!("Move not possible now, move directory yourself")
    }
    let move_time_end = Local::now();
    let move_time = move_time_end - move_time_start;
    info!("Move took {} time", move_time.to_string());
    Ok(())
}
