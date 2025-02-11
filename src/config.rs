impl Config {
    pub fn load(path: &Path)->Result<Self>{
        Ok(serde::from_str(&fs::read_to_string(path)?)?)
    }
}
