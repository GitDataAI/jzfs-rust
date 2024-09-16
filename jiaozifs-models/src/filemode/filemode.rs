use std::fmt;
use std::str::FromStr;
use std::num::ParseIntError;
use std::io;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileMode {
    Empty = 0,
    Dir = 0o0040000,
    Regular = 0o0100644,
    Deprecated = 0o0100664,
    Executable = 0o0100755,
    Symlink = 0o0120000,
    Submodule = 0o0160000,
}

impl FileMode {
    pub fn new(s: &str) -> anyhow::Result<FileMode> {
        let n = u32::from_str_radix(s, 8)?;
        match n {
            0 => Ok(FileMode::Empty),
            0o0040000 => Ok(FileMode::Dir),
            0o0100644 => Ok(FileMode::Regular),
            0o0100664 => Ok(FileMode::Deprecated),
            0o0100755 => Ok(FileMode::Executable),
            0o0120000 => Ok(FileMode::Symlink),
            0o0160000 => Ok(FileMode::Submodule),
            _ => Err(anyhow::anyhow!("Invalid file mode: {}", n)),
        }
    }

    #[cfg(target_os = "linux")]
    pub fn to_os_file_mode(self) -> fs::Permissions {
        use fs::Permissions;
        let mut perm = Permissions::from_mode(0);
        match self {
            FileMode::Dir => perm.set_mode(FileMode::Dir as u32),
            FileMode::Regular => perm.set_mode(FileMode::Regular as u32 & 0o7777),
            FileMode::Deprecated => perm.set_mode(FileMode::Deprecated as u32 & 0o7777),
            FileMode::Executable => perm.set_mode(FileMode::Executable as u32 & 0o7777),
            FileMode::Symlink => perm.set_mode(FileMode::Symlink as u32),
            FileMode::Submodule => perm.set_mode(FileMode::Submodule as u32),
            FileMode::Empty => perm.set_mode(0),
        }
        perm
    }
}

impl fmt::Display for FileMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mode = match *self {
            FileMode::Empty => "0000000",
            FileMode::Dir => "0040000",
            FileMode::Regular => "0100644",
            FileMode::Deprecated => "0100664",
            FileMode::Executable => "0100755",
            FileMode::Symlink => "0120000",
            FileMode::Submodule => "0160000",
        };
        write!(f, "{}", mode)
    }
}

impl FromStr for FileMode {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        FileMode::new(s)
            .map_err(|e|e.into())
            .and_then(|m| Ok(m))
    }
}


#[cfg(test)]
mod filemode_test{
    use super::*;
    #[test]
    fn test_filemode_new(){
        assert_eq!(FileMode::new("0040000").unwrap(), FileMode::Dir);
        assert_eq!(FileMode::new("0100644").unwrap(), FileMode::Regular);
        assert_eq!(FileMode::new("0100664").unwrap(), FileMode::Deprecated);
        assert_eq!(FileMode::new("0100755").unwrap(), FileMode::Executable);
        assert_eq!(FileMode::new("0120000").unwrap(), FileMode::Symlink);
    }
}