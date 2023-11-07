use crate::main;
#[cfg(test)]
mod tests {
    use crate::check_downloader_present;

    #[test]
    fn testion(){
        let result = 2;
        assert_eq!(result,2);
    }
    #[test]
    fn app_present() {
        let result = check_downloader_present();
        assert!(result);
    }
}