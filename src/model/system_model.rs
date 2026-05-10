pub struct FsPath {
    pub fs_path: String,
}

impl FsPath {
    pub fn get_path() -> Self {
        Self {
            fs_path: std::env::var("PROPHECY_NOTES_PATH").expect("[ERROR] Fs path not found"),
        }
    }
}
