use std::path::PathBuf;
use crate::utils::*;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub enum PreviewType {
    #[default]
    Contents,
    TODO,
    README,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub enum CurrentWindow {
    #[default]
    Left,
    Right,
    Dialog,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct App {
    pub exit: bool,
    pub default_editor: String,
    pub dirs: Vec<PathBuf>,
    pub sel_dir: usize,
    pub preview_conts_dirs: Vec<PathBuf>,
    pub preview_scroll: (u16, u16),
    pub preview_file_conts: String,
    pub sel_prev_conts_dir: usize,
    pub sel_window: CurrentWindow,
    pub preview_type: PreviewType,
    pub only_output_path: bool,
    pub custom_cmd: String,
    pub status_txt: String,
    pub undo_vec: Vec<(PathBuf, usize)>,
    pub redo_vec: Vec<(PathBuf, usize)>,
    pub dialogbox: DialogBox,
    pub seach: Search,
}


#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct DialogBox {
    pub dirs: Vec<PathBuf>,
    pub sel_dir: usize,
    pub preview_dirs: Vec<PathBuf>,
    pub back_dirs: Vec<PathBuf>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
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

        for i in self.main_dirs.clone().iter().filter(|d| d.file_name().unwrap().to_str().unwrap().contains(&self.txt)) {
            dirs.push(i.to_path_buf())
        }
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
    pub fn new() -> Self {
        let config_dir_path = dirs::config_dir().unwrap().join("dapu");

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
        }


    } 

    pub fn read_todo_readme(&mut self) {
        if self.preview_type != PreviewType::Contents {
            let string = if self.preview_type == PreviewType::TODO { "TODO" } else { "README" };

            if let Ok(read_dir) = &mut std::fs::read_dir(&self.dirs[self.sel_dir]) {
                if let Some(file) = read_dir.find(|f| f.as_ref().unwrap().file_name().to_str().unwrap().contains(string)) {
                    self.preview_file_conts = std::fs::read_to_string(file.unwrap().path()).unwrap();
                } else {self.preview_file_conts.clear()}
            }
        }
    }

    pub fn update_prev_dirs(&mut self) {
        if let Some(dir) = self.dirs.get(self.sel_dir) {
            match dir.canonicalize() {
                Ok(full_path) => {
                    if let Ok(read_dir) = std::fs::read_dir(full_path.to_str().unwrap()) {
                        self.preview_conts_dirs = 
                            read_dir.map(|f| 
                                         f.unwrap_or_else(|_| {
                                             exit_with_err_msg("No permissions to read file in directory or file dosnt exist");
                                             unreachable!()
                                         }).path()
                                        ).collect();

                        if self.sel_prev_conts_dir > self.preview_conts_dirs.len().saturating_sub(1)  {
                            self.sel_prev_conts_dir = self.preview_conts_dirs.len() - 1
                        }
                    }
                } 
                Err(_) => {
                    self.preview_conts_dirs.clear();
                    self.status_txt = String::from("Path doesn't exist!");
                }
            }
        }
    }

    pub fn save_to_conf(&self) {
        let config_dir_path = dirs::config_dir().unwrap().join("dapu");
        let instance = ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default()).unwrap();
        std::fs::write(config_dir_path.join("dapu.ron"), instance).unwrap();
    }

    pub fn add_remove_dir(add_dir: Option<String>, remove_dir: Option<String>) {
        let config_dir_path = dirs::config_dir().unwrap().join("dapu");
        let config_file_path = dirs::config_dir().unwrap().join("dapu").join("dapu.ron");

        let instance =
            match std::fs::read_to_string(&config_file_path) {
                Ok(config_file_contents) => {
                    let mut instance: App = ron::de::from_str(
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
                    std::fs::create_dir_all(&config_dir_path).unwrap();
                    std::fs::File::create(&config_file_path).unwrap();

                    match (add_dir, remove_dir) {
                        (_, Some(_)) => {
                            println!("Cannot remove dir if there are none added");
                            std::process::exit(0)
                        },
                        (Some(add_dir), _) => {
                            let dir_full_path = PathBuf::from(add_dir).canonicalize().unwrap();
                            Self {
                                dirs: vec![PathBuf::from(dir_full_path)],
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

fn check_for_errs(app: App) -> App {
    let mut app = app;

    if app.dirs.is_empty() {
        exit_with_help_msg("Try to add an directory with `dapu -a .` (the '.' beeing the directory)"); unreachable!()
    }
    if app.sel_dir > app.dirs.len() - 1 { app.sel_dir = 0 };
    if app.exit { app.exit = false }
    app.status_txt.clear();
    app.dialogbox.dirs.clear();
    app.dialogbox.back_dirs.clear();
    if app.sel_window == CurrentWindow::Dialog {
        app.sel_window = CurrentWindow::Left
    }
    app.seach.is_typing = false;
    app.seach.txt.clear();

    app.undo_vec.clear();
    app.redo_vec.clear();

    app
}


