#[cfg(test)]
mod tests {
    use std::fs;
    use crate::{check_downloader_present, move_to_nas};

    #[test]
    fn testion() {
        let result = 2;
        assert_eq!(result, 2);
    }

    #[test]
    fn app_present() {
        let result = check_downloader_present();
        assert!(result);
    }

    #[test]
    fn move_tester() {
        use std::path::Path;
        use std::fs::File;
        let source_dir: String = "test".to_string();
        let source_file: String = "text.txt".to_string();
        let target_dir: String = "test_target\\".to_string();
        let target_dir_extension = format!("{target_dir}*");
        //Setup
        fs::create_dir(source_dir.clone()).expect("Creation source folder failed");
        File::create(format!("{source_dir}/{source_file}")).expect("Could not create test file");
        //Move
        move_to_nas(source_dir.clone(), target_dir_extension.clone());
        //Check if the move has succeeded
        assert!(Path::new(&target_dir).exists());
        assert!(Path::new(&format!("{target_dir}/{source_file}")).exists());
        //Cleanup
        fs::remove_dir_all(target_dir.clone()).expect("Could note remove the target dir");
    }
}