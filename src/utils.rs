/// Contains methods useful for retrieving stuff from the config.


use std::{io::{self}, path::PathBuf, fs};
use crate::login::Login;

// TODO: refactor this later to return a struct named UserConfig.
pub fn retrieve_from_datafile() -> Vec<Login> {
    let full_path = retrieve_config_fpath();

    // either creates the data file or reads from it.
    if full_path.exists() {
        let val = fs::read_to_string(&full_path).unwrap();
        let bytes = val.as_bytes();
        
        // If the file is not empty.
        if bytes.len() > 0 {
            let data: Vec<Login> = bincode::deserialize(val.as_bytes()).unwrap();
            return data;
        }

    } else {
        fs::File::create(full_path).expect("Something went wrong.");
    }

    Vec::new()
}


pub fn write_to_datafile(logins: &Vec<Login>) -> Result<(), io::Error> {
    let path = retrieve_config_fpath();
    let bytes = bincode::serialize(&logins).unwrap();

    fs::write(path, bytes)
}


/// Retrieves the path for the file where stuff is serialized to.
/// It also creates the xPass directory, in case it doesn't exist.
pub fn retrieve_config_fpath() -> PathBuf {
    let conf_dir = dirs::config_dir().unwrap();

    let folder_path = PathBuf::from(format!("{}/xPass/", conf_dir.display()));

    if !folder_path.exists() {
        // Creates the directory.
        fs::create_dir(&folder_path).expect("COULDN'T CREATE DIR");
    }

    folder_path.join("data")
}