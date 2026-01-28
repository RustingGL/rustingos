#![allow(dead_code)]



pub mod fstream {
    use core::result::Result;
    use alloc::string::String;

    fn new(filename: &str, mode: FileMode) -> Result<(ifstream::Ifstream, ofstream::Ofstream, iofstream::Iofstream), String> {
        match mode {
            FileMode::Read => Ok((ifstream::Ifstream::new(String::from(filename)), ofstream::Ofstream::default(), iofstream::Iofstream::default())),
            FileMode::Write => Ok((ifstream::Ifstream::default(), ofstream::Ofstream::new(String::from(filename)), iofstream::Iofstream::default())),
            FileMode::ReadWrite => Ok((ifstream::Ifstream::default(), ofstream::Ofstream::default(), iofstream::Iofstream::new(String::from(filename)))),
        }
    }

    pub mod ifstream {
        use alloc::string::String;
        pub struct Ifstream {
            filename: String,
        }

        impl Ifstream {
            pub fn new(fname: String) -> Self {
                Self {filename: fname}
            }
            
            pub fn default() -> Self {
                Self {filename: String::new()}
            }
        }
    }

    pub mod ofstream {
        use alloc::string::String;
        pub struct Ofstream {
            filename: String,
        }

        impl Ofstream {
            pub fn new(fname: String) -> Self {
                Self {filename: fname}
            }
            
            pub fn default() -> Self {
                Self {filename: String::new()}
            }
        }
    }

    pub mod iofstream {
        use alloc::string::String;
        pub struct Iofstream {
            filename: String,
        }

        impl Iofstream {
            pub fn new(fname: String) -> Self {
                Self {filename: fname}
            }
            
            pub fn default() -> Self {
                Self {filename: String::new()}
            }
        }
    }

    pub enum FileMode {
        Read,
        Write,
        ReadWrite,
    }
}