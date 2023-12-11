use crossterm::event::{KeyCode, Event, self, KeyModifiers};
use crate::{app::{App, PreviewType, CurrentWindow}, tui::Tui};
use std::io::Result;


pub fn update(app: &mut App, _tui: &mut Tui) -> Result<()> {
    if let Event::Key(key) = event::read()? {
        match key.code {
            KeyCode::Char('q') =>{app.save_to_conf(); app.exit = true},

            KeyCode::Char('o') => app.only_output_path = !app.only_output_path,

            KeyCode::Char('c') => app.preview_type = PreviewType::Contents,
            KeyCode::Char('t') => app.preview_type = PreviewType::TODO,
            KeyCode::Char('r') => app.preview_type = PreviewType::README,


            KeyCode::Enter => {
                app.save_to_conf();
                app.exit = true;

                let path = 
                    match app.sel_window {
                        CurrentWindow::Right => &app.preview_conts_dirs[app.sel_prev_conts_dir],
                        CurrentWindow::Left => &app.dirs[app.sel_dir],
                    };
                
                let cmd = app.custom_cmd.replace("{}", path.to_str().unwrap());

                if key.modifiers == KeyModifiers::ALT { // custom command to exec on dir
                    std::process::Command::new("sh").arg("-c").arg(&cmd).status().unwrap();

                } else {
                    if app.only_output_path {
                        match path.is_dir() {
                            true => println!("{}", path.to_str().unwrap()),
                            false => println!("{}", path.parent().unwrap().to_str().unwrap())
                        }
                    } else {
                        std::process::Command::new("nvim").arg(path).status().unwrap();
                    }
                }

            }

            _ => {
                // binds for preview window / right
                if let CurrentWindow::Right = app.sel_window {
                    match key.code {
                        KeyCode::Char('j') | KeyCode::Down => {
                            app.sel_prev_conts_dir = (app.sel_prev_conts_dir + 1) % app.preview_conts_dirs.len()
                        }

                        KeyCode::Char('k') | KeyCode::Up => {
                            app.sel_prev_conts_dir = match app.sel_prev_conts_dir {
                                0 => app.preview_conts_dirs.len() - 1,
                                _ => app.sel_prev_conts_dir - 1 
                            }
                        }

                        KeyCode::Char('h') | KeyCode::Left => app.sel_window = CurrentWindow::Left,

                        _ => ()
                    }
                }

                // binds for left / main window
                if let CurrentWindow::Left = app.sel_window {
                    match key.code {
                        KeyCode::Char('j') | KeyCode::Down => {
                            app.sel_dir = (app.sel_dir + 1) % app.dirs.len();

                            app.git_pull_out.clear();
                        }

                        KeyCode::Char('k') | KeyCode::Up => {
                            app.sel_dir = match app.sel_dir {
                                0 => app.dirs.len() - 1,
                                _ => app.sel_dir - 1 
                            };

                            app.git_pull_out.clear();
                        }

                        KeyCode::Char('l') | KeyCode::Right => app.sel_window = CurrentWindow::Right,

                        KeyCode::Char('f') => {
                            std::env::set_current_dir(&app.dirs[app.sel_dir]).unwrap();
                            std::process::Command::new("git").arg("fetch").output().unwrap();
                        }

                        KeyCode::Char('p') => {
                            std::env::set_current_dir(&app.dirs[app.sel_dir]).unwrap();
                            let out = std::process::Command::new("git").arg("pull").output().unwrap();
                            let stdout = out.stdout;
                            let stderr = out.stderr;
                            
                            app.git_pull_out = String::from_utf8(stdout).unwrap().replace("\n", " ") +  &String::from_utf8(stderr).unwrap().replace("\n", " ")
                        }
                        

                        _ => ()
                    }
                }

            } // match block else

        } // match key.code
    } // is there key event
  Ok(())
}
