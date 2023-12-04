use std::path::PathBuf;
use crate::utils::*;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub enum PreviewType {
    #[default]
    Contents,
    TODO,
    README,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub enum MainWindows {
    #[default]
    Left,
    Right,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct App {
    pub exit: bool,
    pub dirs: Vec<PathBuf>,
    pub sel_dir: usize,
    pub sel_window: MainWindows,
    pub preview_type: PreviewType,
    pub only_output_path: bool,
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
                                only_output_path: true, // REMOVE!!!!!
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

fn check_for_errs(app: App) -> App {
    let mut app = app;

    if app.dirs.is_empty() {
        exit_with_help_msg("Try to add an directory with `dapu -a .` (the '.' beeing the directory)"); unreachable!()
    }
    if app.sel_dir > app.dirs.len() - 1 { app.sel_dir = 0 };
    if app.exit { app.exit = false }

    app
}
