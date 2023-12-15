use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use crate::utils::*;

#[derive(Serialize, Deserialize, Default, Clone, PartialEq)]
pub enum PreviewType {
    #[default]
    Contents,
    TODO,
    README,
}

#[derive(Default, PartialEq)]
pub enum CurrentWindow {
    #[default]
    Left,
    Right,
    Dialog,
}

#[derive(Default)]
pub struct App {
    pub exit: bool,
    pub default_editor: String,
    pub dirs: Vec<PathBuf>,
    pub sel_dir: usize,
    pub sel_window: CurrentWindow,
    pub preview_type: PreviewType,
    pub only_output_path: bool,
    pub custom_cmd: String,
    pub status_txt: String,
    pub undo_vec: Vec<(PathBuf, usize)>,
    pub redo_vec: Vec<(PathBuf, usize)>,
    pub prev: Preview,
    pub dialogbox: DialogBox,
    pub seach: Search,
}

#[derive(Default)]
pub struct Preview {
    pub dirs: Vec<PathBuf>,
    pub scroll: (u16, u16),
    pub file_txt: String,
    pub sel_dir: usize,
}


#[derive(Serialize, Deserialize, Default)]
struct Config {
    dirs: Vec<PathBuf>,
    sel_dir: usize,
    preview_type: PreviewType,
    default_editor: String,
    custom_cmd: String,
    only_output_path: bool,
}


#[derive(Default)]
pub struct DialogBox {
    pub dirs: Vec<PathBuf>,
    pub sel_dir: usize,
    pub preview_dirs: Vec<PathBuf>,
    pub back_dirs: Vec<PathBuf>,
}

#[derive(Default)]
pub struct Search {
    pub is_typing: bool, 
    pub txt: String,
    pub main_dirs: Vec<PathBuf>,
}

impl Search {
    pub fn seach_from_dirs(&mut self, dirs: &mut Vec<PathBuf>) {
        if self.main_dirs.is_empty() {
            self.main_dirs = dirs.clone();
        }
        dirs.clear();

        self.main_dirs.clone()
            .iter()
            .filter(|d|
                d.file_name().unwrap()
                    .to_str().unwrap()
                    .to_lowercase()
                    .contains(&self.txt)
            ).for_each(|dir| {
                dirs.push(dir.to_path_buf())
            });
    }

    pub fn exit(&mut self) {
        self.txt.clear();
        self.is_typing = false;
    }
    pub fn revert_dirs(&mut self, dirs: &mut Vec<PathBuf>) {
        *dirs = self.main_dirs.clone();
        self.main_dirs.clear();
    }
}

impl DialogBox {
    pub fn go_back_dir(&mut self) {
        if !self.back_dirs.is_empty() {
            self.dirs = std::fs::read_dir(self.back_dirs.pop().unwrap()).unwrap()
                .map(|f| f.unwrap().path())
                .filter(|f| f.is_dir())
                .collect();
        }
    }
    
    pub fn go_forward_dir(&mut self) {
        if has_subdirectories(&self.dirs[self.sel_dir]) {
            self.back_dirs.push(
                self.dirs[self.sel_dir]
                .parent().unwrap()
                .to_path_buf()
            );

            self.dirs = std::fs::read_dir(&self.dirs[self.sel_dir]).unwrap()
                .map(|f| f.unwrap().path())
                .filter(|f| f.is_dir())
                .collect();

        }
    }
    
    pub fn open_home_dir(&mut self) {
        if self.dirs.is_empty() {
            self.dirs = std::fs::read_dir(dirs::home_dir().unwrap()).unwrap()
                .map(|f| f.unwrap().path())
                .filter(|f| f.is_dir())
                .collect();
        }
    }

    pub fn update_prev_dirs(&mut self) {
        if self.sel_dir > self.dirs.len().saturating_sub(1) {
            self.sel_dir = self.dirs.len().saturating_sub(1)
        }

        match self.dirs[self.sel_dir].canonicalize() {
            Ok(full_path) => {
                if let Ok(read_dir) = std::fs::read_dir(full_path.to_str().unwrap()) {
                    self.preview_dirs= 
                        read_dir.map(|f| 
                            f.unwrap_or_else(|_| {
                                exit_with_err_msg("No permissions to read file in directory or file dosnt exist");
                                unreachable!()
                            }).path()
                        ).collect();
                }

            } 
            Err(_) => {
                self.preview_dirs.clear();
            }
        }
    }
}

impl App {
    pub fn new(only_path: bool) -> Self {
        let config_dir_path = dirs::config_dir().unwrap().join("dapu");

        let mut instance =
            if std::fs::read_dir(&config_dir_path).is_err() | std::fs::read(config_dir_path.join("dapu.ron")).is_err() {
                exit_with_help_msg("Try to add an directory with `dapu -a .` (the '.' beeing the directory)");
                unreachable!()
            } else {
                check_for_errs(
                    ron::de::from_str(
                        &std::fs::read_to_string(config_dir_path.join("dapu.ron")).unwrap()
                    ).unwrap_or_else(|_| {
                        exit_with_err_msg("dapu.ron is invalid, try to delete it or edit to make it valid");
                        unreachable!() 
                    })
                )
            };

        instance.update_prev_dirs();
        if only_path {
            instance.only_output_path = only_path
        }

        instance
    } 

    pub fn read_todo_readme(&mut self) {
        if self.preview_type != PreviewType::Contents {
            let string = if self.preview_type == PreviewType::TODO { "TODO" } else { "README" };

            if let Ok(read_dir) = &mut std::fs::read_dir(&self.dirs[self.sel_dir]) {
                if let Some(file) = read_dir.find(|f| f.as_ref().unwrap().file_name().to_str().unwrap().contains(string)) {
                    self.prev.file_txt = std::fs::read_to_string(file.unwrap().path()).unwrap();
                } else {self.prev.file_txt.clear()}
            }
        }
    }

    pub fn update_prev_dirs(&mut self) {
        if let Some(dir) = self.dirs.get(self.sel_dir) {
            match dir.canonicalize() {
                Ok(full_path) => {
                    if let Ok(read_dir) = std::fs::read_dir(full_path.to_str().unwrap()) {
                        self.prev.dirs = 
                            read_dir.map(|f| 
                                         f.unwrap_or_else(|_| {
                                             exit_with_err_msg("No permissions to read file in directory or file dosnt exist");
                                             unreachable!()
                                         }).path()
                                        ).collect();

                        if self.prev.sel_dir > self.prev.dirs.len().saturating_sub(1)  {
                            self.prev.sel_dir = self.prev.dirs.len() - 1
                        }
                    }
                } 
                Err(_) => {
                    self.prev.dirs.clear();
                    self.status_txt = String::from("Path doesn't exist!");
                }
            }
        }
    }

    pub fn add_remove_dir(add_dir: Option<String>, remove_dir: Option<String>) {
        let config_dir_path = dirs::config_dir().unwrap().join("dapu");
        let config_file_path = dirs::config_dir().unwrap().join("dapu").join("dapu.ron");

        let instance =
            match std::fs::read_to_string(&config_file_path) {
                Ok(config_file_contents) => {
                    let mut instance: Config = ron::de::from_str(
                        &config_file_contents
                    ).unwrap();

                    if let Some(add_dir) = add_dir {
                        let add_dir_full_path = PathBuf::from(add_dir).canonicalize().unwrap_or_else(|_|{
                            exit_with_err_msg("Path doesn't exits"); unreachable!()});
                        if !add_dir_full_path.is_dir() {exit_with_err_msg("Path not directory")}
                        match instance.dirs.contains(&add_dir_full_path) {
                            true => exit_with_err_msg("Directory already in dapu"),
                            false => instance.dirs.push(add_dir_full_path)
                        }
                    }

                    if let Some(remove_dir) = remove_dir {
                        let remove_dir_full_path = PathBuf::from(remove_dir).canonicalize().unwrap_or_else(|_|{
                            exit_with_err_msg("Path doesn't exits"); unreachable!()});
                        if !remove_dir_full_path.is_dir() {exit_with_err_msg("Path not directory")}

                        instance.dirs.remove(
                            match instance.dirs.iter().position(|f| f == &remove_dir_full_path) {
                                Some(pos) => pos,
                                None => {
                                    exit_with_err_msg("Directory not in dapu"); 
                                    unreachable!()
                                }
                            }
                        );
                    }
                    instance
                }

                Err(_) => {
                    match (add_dir, remove_dir) {
                        (_, Some(_)) => {
                            println!("Cannot remove dir if there are none added");
                            std::process::exit(0)
                        },
                        (Some(add_dir), _) => {
                            let add_dir_full_path = PathBuf::from(add_dir).canonicalize().unwrap_or_else(|_|{
                                exit_with_err_msg("Path doesn't exits"); unreachable!()});
                            if !add_dir_full_path.is_dir() {exit_with_err_msg("Path not directory")}

                            std::fs::create_dir_all(&config_dir_path).unwrap();
                            std::fs::File::create(&config_file_path).unwrap();

                            Config {
                                dirs: vec![add_dir_full_path],
                                default_editor: String::from("nvim"),
                                ..Default::default()
                            }
                        }
                        _ => unreachable!(),
                    }

                }
            };

        std::fs::write(config_file_path,
           ron::ser::to_string_pretty(
               &instance,
               ron::ser::PrettyConfig::default()
           ).unwrap()
        ).unwrap();
    }

    pub fn save_to_conf(&self) {
        let config_dir_path = dirs::config_dir().unwrap().join("dapu");
        let instance = Config { 
            dirs: self.dirs.clone(),
            sel_dir: self.sel_dir,
            default_editor: self.default_editor.clone(),
            custom_cmd: self.custom_cmd.clone(),
            only_output_path: self.only_output_path,
            preview_type: self.preview_type.clone(),
        };
        let instance = ron::ser::to_string_pretty(&instance, ron::ser::PrettyConfig::default()).unwrap();
        std::fs::write(config_dir_path.join("dapu.ron"), instance).unwrap();
    }
}


fn has_subdirectories(path: &PathBuf) -> bool {
    if let Ok(files) = std::fs::read_dir(path) {
        for f in files {
            if let Ok(f) = f {
                if f.file_type().map_or(false, |ft| ft.is_dir()) {
                    return true;
                }
            }
        }
    }
    false
}

fn check_for_errs(conf: Config) -> App {
    let mut conf = conf;

    if conf.dirs.is_empty() {
        exit_with_help_msg("Try to add an directory with `dapu -a .` (the '.' beeing the directory)"); unreachable!()
    }
    if conf.sel_dir > conf.dirs.len() - 1 { conf.sel_dir = 0 };

    App {
        dirs: conf.dirs,
        sel_dir: conf.sel_dir,
        default_editor: conf.default_editor,
        custom_cmd: conf.custom_cmd,
        only_output_path: conf.only_output_path,
        preview_type: conf.preview_type,
        ..Default::default()
    }
}
