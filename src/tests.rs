#[cfg(test)]
mod tests {
    use crate::{check_downloader_present, evaluate_move_path, move_to_nas};
    use std::fs;
    use crate::logging::initialize_logging;

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
        assert!(file_exists, "Target file in the target directory was not createed");
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
        assert_eq!("M:/media/youtube/", result_path);
    }

    #[test]
    fn move_path_evaluation_no_path_os_macos() {
        let test_target = "";
        let os_type = "macos";
        let result_path = evaluate_move_path(os_type, test_target.to_string());
        assert_eq!("/Volumes/huge/media/youtube/", result_path);
    }

    #[test]
    fn move_with_fs_extra() {
        use fs_extra::move_items;
        use std::fs::File;
        use std::path::Path;

        let source_dir: String = "test_source_fs_extra".to_string();
        let source_file: String = "text_fs_extra.txt".to_string();
        let target_dir: String = "test_target_fs_extra".to_string();

        //Create source dir
        fs::create_dir(source_dir.clone()).expect("Creation source folder failed");
        //Put a file in the source dir
        File::create(format!("{}/{source_file}", source_dir.clone()))
            .expect("Could not create test file");
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
}
