use std::ffi::CString;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::{Error, ErrorKind, Read};

pub fn load_cstring(file_path: &str) -> Result<CString, Error> {
    let exe_file_name = std::env::current_exe()?;

    let exe_path = exe_file_name.parent().ok_or(Error::new(ErrorKind::NotFound, "Path not found"))?;

    println!("{:?}", exe_path);
    let mut file = File::open(
        str_location_to_path(exe_path, file_path)
    )?;

    // allocate buffer of the same size as file
    let mut buffer: Vec<u8> = Vec::with_capacity(
        file.metadata()?.len() as usize + 1
    );
    file.read_to_end(&mut buffer)?;

    // check for nul type
    if buffer.iter().find(|i| **i == 0).is_some() {
        return Err(Error::new(ErrorKind::InvalidData, "Data contains nul!"));
    }

    Ok(unsafe { CString::from_vec_unchecked(buffer)})
}

fn str_location_to_path(root_dir: &Path, location: &str) -> PathBuf {
    let mut path: PathBuf = root_dir.into();

    for part in location.split("/") {
        path = path.join(part);
    }

    path
}