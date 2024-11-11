// ficus is a library for managing files and directories.

use ficus::FileSystem;

fn main() {

    let mut fs = FileSystem::new();

    fs.create_folder(String::from("test folder"), None);

    println!("{:?}", fs.search_folders(String::from("t"), None));
}
