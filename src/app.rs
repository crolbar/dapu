use std::{path::PathBuf, io::Read};
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub enum PreviewType {
    Contents,
    TODO,
    README,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct App {
    pub exit: bool,
    pub dirs: Vec<PathBuf>,
    pub sel_dir: usize,
    pub sel_window: usize,
    pub preview_type: PreviewType,

    // only way (I can think of) to change working directory is with an alias
    // so after you exit out of vim you are in the project directory
    pub has_alias: bool,
}

impl App {
    pub fn new() -> Self {
        let config_dir_path = dirs::config_dir().unwrap().join("dapu");

        if std::fs::read_dir(&config_dir_path).is_err() {
            println!("{}Try to add an directory with `dapu -a .` (the '.' beeing the directory)", "\x1b[34m");
            std::process::exit(0);
        } else {
            let conts = std::fs::read(config_dir_path.join("dapu.ron")).unwrap();
            ron::de::from_str(&String::from_utf8(conts).unwrap()).unwrap()
        }


    } 

    pub fn save_to_conf(&self) {
        let config_dir_path = dirs::config_dir().unwrap().join("dapu");
        let instance = ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default()).unwrap();
        std::fs::write(config_dir_path.join("dapu.ron"), instance).unwrap();
    }

    pub fn add_remove_dir(add_dir: Option<String>, remov_dir: Option<String>) {
        let config_dir_path = dirs::config_dir().unwrap().join("dapu");
        let config_file_path = dirs::config_dir().unwrap().join("dapu").join("dapu.ron");

        if std::fs::read_dir(&config_dir_path).is_err() {
            std::fs::create_dir_all(&config_dir_path).unwrap();
            std::fs::File::create(&config_file_path).unwrap();
        } 
        
        let mut file = std::fs::File::open(&config_file_path).unwrap();
        let mut conf_file_contents = String::new();
        file.read_to_string(&mut conf_file_contents).unwrap();

        let instance = if conf_file_contents.is_empty(){
            match (add_dir, remov_dir) {
                (_, Some(_)) => {println!("cannot remove dir if there are 0"); std::process::exit(0)},
                (Some(add_dir), _) => {
                    let dir_full_path = PathBuf::from(add_dir).canonicalize().unwrap();
                    Self {
                        exit: false,
                        sel_dir: 0,
                        sel_window: 0,
                        preview_type: PreviewType::Contents,
                        has_alias: false,
                        dirs: vec![PathBuf::from(dir_full_path)],
                    }
                }
                _ => unreachable!(),
                
            }
        } else {
            let s = std::fs::read_to_string(&config_file_path).unwrap();
            let mut conts: App = ron::de::from_str(&s).unwrap();

            let dir_full_path = PathBuf::from(add_dir.unwrap()).canonicalize().unwrap();

            conts.dirs.push(PathBuf::from(dir_full_path));

            conts

        };

        std::fs::write(config_file_path,
           ron::ser::to_string_pretty(
               &instance,
               ron::ser::PrettyConfig::default()
           ).unwrap()
       ).unwrap();
    }
}
