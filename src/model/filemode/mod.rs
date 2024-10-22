use lazy_static::lazy_static;

pub type FileMode = u32;

lazy_static!(
    pub static ref Empty: FileMode = 0;
    pub static ref Dir: FileMode = 16384;
    pub static ref Regular: FileMode = 33188;
    pub static ref Deprecated: FileMode = 33204;
    pub static ref Executable: FileMode = 33261;
    pub static ref Symlink: FileMode = 40960;
    pub static ref Submodule: FileMode = 57344;
);