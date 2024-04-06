use std::{env, ffi::CString, fs::{read_dir, File}, io::Write, path::PathBuf};
use dialoguer::Select;

struct InitFile{
    provider:String,
    url: String, 
    password: String, 
    files: Vec<PathBuf>
}

impl InitFile {
    fn new(provider: &str, url: &str, password: &str, files: Vec<PathBuf>) -> Self {
        Self {provider: provider.into(),  url: url.into(), password: password.into(), files}
    }
    fn init(&mut self) {
        self.files = read_all_files(env::current_dir().unwrap())
    }
    fn upload() {
        
    }
}

fn main() {
    let mut file = InitFile::new("Google", "test", "test", [].to_vec());

    file.init();

}

fn read_all_files(path:PathBuf) -> Vec<PathBuf> {
    let mut paths:Vec<PathBuf> = Vec::new();
    for entry in read_dir(path).expect("Error reading") {
        match entry {
            Ok(entry) => if !entry.path().is_dir() {
                paths.push(entry.path());
            } else {
                paths.extend(read_all_files(entry.path()));
            }
                    
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    return paths;
}

fn dir_init() {
    let fs = File::create("init.json");
    
    match fs {
        Ok(mut file) => {
            for path in read_all_files(env::current_dir().unwrap()).iter() {
                let buf_path = CString::new(path.to_str().unwrap());
                file.write(buf_path.unwrap().as_bytes());            
            }
        }

        Err(e)=> eprint!("Error: {}", e)
    }
}

fn init() {
 let providers = vec![
        "\x1b[93mAWS Bucket\x1b[0m", 
        "\x1b[94mGoogle Cloud\x1b[0m", 
        "\x1b[96mAzure Blob\x1b[0m", 
        "\x1b[91mOracle Storage\x1b[0m"
    ];

    let selection = Select::new()
        .with_prompt("Select a provider")
        .items(&providers)
        .interact()
        .unwrap();
}
    