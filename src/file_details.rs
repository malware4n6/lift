pub mod file_details {
    use std::{fs, os::linux::fs::MetadataExt as _, path::Path};

    use magic::{cookie::Load, Cookie};

    use bytesize::*;
    use colored::*;

    pub struct FileDetails {
        cookie: Option<Cookie<Load>>,
    }

    impl FileDetails {
        pub fn new() -> Self {
            let mut cookie: Option<Cookie<Load>> = None;
            match magic::Cookie::open(magic::cookie::Flags::ERROR) {
                Ok(c) => {
                    cookie = Some(c.load(&Default::default()).expect(""));
                }
                Err(_e) => {
                    print!("magic cannot be loaded");
                }
            }
            FileDetails { cookie }
        }

        fn print_type(&self, path: &Path) {
            if path.is_symlink() {
                print!("{}", "l ".green());
            } else if path.is_file() {
                print!("f ");
            } else if path.is_dir() {
                print!("{}", "d ".blue());
            } else {
                print!("{}", "? ".yellow());
            }
        }

        fn print_size(&self, path: &Path) {
            let size: u64 = match path.metadata() {
                Ok(m) => m.st_size(),
                Err(_e) => 0,
            };
            let humansize = ByteSize::b(size);
            print!("{:10}\t{:10}\t", size, humansize.to_string_as(false));
        }

        fn print_name_file(&self, path: &Path, filename: &String, shortname: bool) {
            // let's consider it is a file (no specific code dedicated to symlink yet)
            if !shortname || filename.len() <= 20 {
                print!("{}", filename);
            } else {
                let stem = path.file_stem().unwrap().to_str().unwrap();
                if stem.len() > 20 {
                    if path.parent().is_some() {
                        print!("{}/", path.parent().unwrap().to_str().unwrap());
                    }
                    print!("{}**", &stem[0..8]);
                    if path.extension().is_some() {
                        print!(".{}", path.extension().unwrap().to_str().unwrap())
                    }
                } else {
                    print!("{}", filename);
                }
            }
        }

        fn print_name(&self, path: &Path, filename: &String, shortname: bool) {
            if path.is_dir() {
                print!("{}", filename.blue());
            } else {
                self.print_name_file(path, filename, shortname);
            }
        }

        fn print_magic(&self, filename: &String) {
            // type if found
            if let Some(cookie) = &self.cookie {
                let magic = cookie.file(filename).expect("libmagic failed");
                if magic.starts_with("PE32")
                    || magic.starts_with("ELF ")
                    || magic.starts_with("setuid ELF ")
                    || magic.starts_with("setgid ELF ")
                {
                    print!("\t{}", magic.purple());
                } else {
                    print!("\t{}", magic);
                }
            }
        }

        pub fn show(&self, filename: &String, recursive: bool, shortname: bool) {
            let path = Path::new(filename);
            if path.exists() {
                self.print_type(path);
                self.print_size(path);
                self.print_name(path, filename, shortname);
                self.print_magic(filename);
                println!();
                if recursive && path.is_dir() {
                    let mut paths: Vec<_> =
                        fs::read_dir(path).unwrap().map(|r| r.unwrap()).collect();
                    paths.sort_by_key(|entry| entry.path());
                    for path in paths {
                        let p = format!("{}", path.path().display());
                        self.show(&p, recursive, shortname);
                    }
                }
            } else {
                println!("{} {:10}\t{}\t{}", "!".red(), -1, -1, filename.red());
            }
        }
    }
}
