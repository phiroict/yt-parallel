#[cfg(test)]
mod tests {
    use crate::logging::initialize_logging;
    use crate::{check_downloader_present, evaluate_move_path, move_to_nas, render_duration_readable};
    use std::fs;
    use chrono::Duration;

    #[test]
    fn app_present() {
        let result = check_downloader_present("yt-dlp".to_string());
        assert!(result);
    }

    #[test]
    fn app_not_present() {
        let result = check_downloader_present("IAmNotThere".to_string());
        assert!(!result);
    }
    #[test]
    fn move_tester() {
        use std::fs::File;
        use std::path::Path;
        let source_dir: String = "test".to_string();
        let source_file: String = "text.txt".to_string();
        let target_dir: String = "test_target".to_string();
        initialize_logging("trace".to_string());
        //Setup
        fs::create_dir(source_dir.clone()).expect("Creation source folder failed");
        File::create(format!("{source_dir}/{source_file}")).expect("Could not create test file");
        //Move

        move_to_nas(source_dir.clone(), target_dir.clone());

        //Check if the move has succeeded
        let dir_exists = Path::new(&target_dir).exists();
        let file_exists = Path::new(&format!("{target_dir}/{source_dir}/{source_file}")).exists();

        //Cleanup
        fs::remove_dir_all(target_dir.clone()).expect("Could note remove the target dir");
        //Process assertions, note that we do the cleanup first and then handle the results as a failed assert will stop the teardown to run.
        assert!(dir_exists, "Target directory was not created");
        assert!(
            file_exists,
            "Target file in the target directory was not createed"
        );
    }

    #[test]
    fn move_path_evaluation_pass_path() {
        let test_target = "/usr/local/bin/test";
        let os_type = "windows";
        let result_path = evaluate_move_path(os_type, test_target.to_string());
        assert_eq!(test_target, result_path);
    }

    #[test]
    fn move_path_evaluation_no_path_os_windows() {
        let test_target = "";
        let os_type = "windows";
        let result_path = evaluate_move_path(os_type, test_target.to_string());
        assert_eq!("M:/youtube/", result_path);
    }

    #[test]
    fn move_path_evaluation_no_path_os_macos() {
        let test_target = "";
        let os_type = "macos";
        let result_path = evaluate_move_path(os_type, test_target.to_string());
        assert_eq!("/Volumes/huge/media/youtube/", result_path);
    }

    #[ignore]
    #[test]
    fn move_with_fs_extra() {
        use fs_extra::move_items;
        use std::fs::File;
        use std::path::Path;

        let source_dir: String = "test_source_fs_extra".to_string();
        let source_file: String = "text_fs_extra.txt".to_string();
        let target_dir: String = "test_target_fs_extra".to_string();

        //Create source dir
        let create_dir_result = fs::create_dir(source_dir.clone());
        match create_dir_result {
            Ok(_) => {}
            Err(e) => {
                println!("Create folder failed, may already exist: {}", e.to_string())
            }
        }
        //Put a file in the source dir
        let file_create_result = File::create(format!("{}/{source_file}", source_dir.clone()));
        match file_create_result {
            Ok(_) => {}
            Err(e) => {
                println!(
                    "Create file failed, continue as it may already exist: {}",
                    e.to_string()
                )
            }
        }
        //Do move
        let options = fs_extra::dir::CopyOptions::new();

        println!("Moving {} to {}", source_dir.clone(), target_dir.clone());
        let demo_source = Path::new(&source_dir);
        if demo_source.exists() {
            println!("Source {} exists ", source_dir.clone());
        } else {
            println!("Source {} does not exist ", source_dir.clone());
            assert!(false);
        }
        let demo_target = Path::new(&target_dir);
        if !demo_target.exists() {
            fs::create_dir_all(demo_target).expect("Could not create target ");
        }
        let move_result = move_items(&[demo_source], demo_target, &options);
        match move_result {
            Ok(_) => {}
            Err(e) => {
                fs::remove_dir_all(source_dir.clone()).expect("Could note remove the source dir");
                assert!(false, "Move failed {e}")
            }
        }

        //Check if the move has succeeded
        assert!(Path::new(&target_dir).exists());
        assert!(Path::new(&format!("{target_dir}/{source_dir}/{source_file}")).exists());
        //Cleanup
        fs::remove_dir_all(target_dir.clone()).expect("Could note remove the target dir");
    }

    #[test]
    fn get_part_files_and_delete_them() {
        use std::fs::File;
        use std::path::Path;

        let source_dir: String = "test_source_fs_extra".to_string();
        let source_file: String = "text_fs_extra.txt".to_string();
        let source_file_part: String = "text_fs_extra.part".to_string();

        //Create source dir
        let create_dir = fs::create_dir(source_dir.clone());
        match create_dir {
            Ok(_) => {}
            Err(e) => {
                println!("Did already exist? We push on: {}", e.to_string())
            }
        }
        //Put a file in the source dir
        File::create(format!("{}/{source_file}", source_dir.clone()))
            .expect("Could not create test file");
        File::create(format!("{}/{source_file_part}", source_dir.clone()))
            .expect("Could not create test file");

        let files = fs::read_dir(Path::new(&source_dir)).unwrap();
        for x in files {
            let name = x.unwrap().path().display().to_string();
            if name.ends_with("part") {
                println!(
                    "Found file in dir {}, removing file {}",
                    source_dir.clone(),
                    name
                );
                fs::remove_file(Path::new(&format!("{}", name))).unwrap();
            }
        }
        let part_file_removed =
            Path::new(&format!("{}/{}", source_dir.clone(), source_file_part)).exists();
        let file_remains = Path::new(&format!("{}/{}", source_dir.clone(), source_file)).exists();
        fs::remove_dir_all(source_dir.clone()).unwrap();
        assert!(!part_file_removed);
        assert!(file_remains);
    }

    #[test]
    fn test_one_hour() {
        let duration = Duration::hours(1);
        assert_eq!(render_duration_readable(duration), "01:00:00");
    }

    #[test]
    fn test_one_minute_one_second() {
        let mut duration = Duration::minutes(1);
        duration = duration +  Duration::seconds(1);
        assert_eq!(render_duration_readable(duration), "00:01:01");
    }

    #[test]
    fn test_one_minute_61_seconds() {
        let mut duration = Duration::minutes(1);
        duration = duration +  Duration::seconds(61);
        assert_eq!(render_duration_readable(duration), "00:02:01");
    }
}
