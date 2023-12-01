#[cfg(test)]
mod tests {
    use crate::{check_downloader_present, evaluate_move_path, move_to_nas};
    use std::fs;

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
        use std::env;
        use std::fs::File;
        use std::path::Path;
        let source_dir: String = "test".to_string();
        let source_file: String = "text.txt".to_string();
        let target_dir: String = "test_target\\".to_string();
        let target_dir_extension = format!("{target_dir}*");
        //Setup
        fs::create_dir(source_dir.clone()).expect("Creation source folder failed");
        File::create(format!("{source_dir}/{source_file}")).expect("Could not create test file");
        //Move
        if "windows".eq(env::consts::OS) {
            move_to_nas(source_dir.clone(), target_dir_extension.clone());
        } else {
            move_to_nas(source_dir.clone(), target_dir.clone());
        }

        //Check if the move has succeeded
        assert!(Path::new(&target_dir).exists());
        assert!(Path::new(&format!("{target_dir}/{source_file}")).exists());
        //Cleanup
        fs::remove_dir_all(target_dir.clone()).expect("Could note remove the target dir");
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
}
