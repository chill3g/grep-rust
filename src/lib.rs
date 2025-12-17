pub mod lib{
    pub fn parse_config(args: &[String]) -> (&str, &str){
        let query = &args[1];
        let file_path = &args[2];

        (query, file_path)
    }
}
