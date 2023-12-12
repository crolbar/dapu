use crossterm::event::{KeyCode, Event, self, KeyModifiers};
use crate::{app::{App, PreviewType, CurrentWindow}, tui::Tui};
use std::io::Result;


pub fn update(app: &mut App, _tui: &mut Tui) -> Result<()> {
    if let Event::Key(key) = event::read()? {
        match key.code {
            KeyCode::Char('q') => {app.save_to_conf(); app.exit = true},
            KeyCode::Esc => {app.save_to_conf(); app.exit = true},

            KeyCode::Char('a') => {
                if app.sel_window != CurrentWindow::Dialog {
                    app.dialogbox.open_home_dir();
                    app.dialogbox.update_prev_dirs();
                    app.sel_window = CurrentWindow::Dialog;
                } else {
                    app.sel_window = CurrentWindow::Left;
                }
            },

            KeyCode::Char('o') => app.only_output_path = !app.only_output_path,

            KeyCode::Char('c') => {
                app.preview_type = PreviewType::Contents;
                app.update_prev_dirs();
            },
            KeyCode::Char('t') => {
                app.preview_type = PreviewType::TODO;
                app.read_todo_readme();
            },
            KeyCode::Char('r') => {
                app.preview_type = PreviewType::README;
                app.read_todo_readme();
            },


            KeyCode::Enter => {
                app.save_to_conf();
                app.exit = true;

                let path = 
                    match app.sel_window {
                        CurrentWindow::Right => &app.preview_conts_dirs[app.sel_prev_conts_dir],
                        CurrentWindow::Left => &app.dirs[app.sel_dir],
                        CurrentWindow::Dialog => &app.dialogbox.dirs[app.dialogbox.sel_dir],
                    };

                // if in dialogbox add dir to main dirs vector
                if app.sel_window == CurrentWindow::Dialog {
                    app.exit = false;
                    if !app.dirs.contains(path) {
                        app.dirs.push(path.to_path_buf())
                    }
                } else 
                 
                // custom command to exec on dir
                if key.modifiers == KeyModifiers::ALT { 
                    let cmd = 
                        match app.custom_cmd.get(0..1) {
                            Some("!") => {
                                app.exit = false;

                                app.custom_cmd
                                    .replace("{}", path.to_str().unwrap())
                                    .replacen("!", "", 1)
                            },

                            _ => app.custom_cmd
                                .replace("{}", path.to_str().unwrap())
                        };
                    std::process::Command::new("sh").arg("-c").arg(&cmd).status().unwrap();

                // normal enter if only path output only path if not open with editor
                } else {
                    match app.only_output_path {
                        true => {
                            match path.is_dir() {
                                true => println!("{}", path.to_str().unwrap()),
                                false => println!("{}", path.parent().unwrap().to_str().unwrap())
                            }
                        }
                        false => {
                            std::process::Command::new(&app.default_editor).arg(path).status().unwrap();
                        }
                    }
                }
            }

            _ => {
                // binds for dialog box 
                if let CurrentWindow::Dialog = app.sel_window {
                    match key.code {
                        KeyCode::Char('j') => {
                            app.dialogbox.sel_dir = (app.dialogbox.sel_dir + 1) % app.dialogbox.dirs.len();

                            app.dialogbox.update_prev_dirs();
                        }

                        KeyCode::Char('k') => {
                            app.dialogbox.sel_dir = match app.dialogbox.sel_dir {
                                0 => app.dialogbox.dirs.len() - 1,
                                _ => app.dialogbox.sel_dir - 1 
                            };
                            app.dialogbox.update_prev_dirs();
                        }

                        KeyCode::Char('G') => {
                            app.dialogbox.sel_dir = app.dialogbox.dirs.len() - 1;

                            app.dialogbox.update_prev_dirs();
                        },
                        KeyCode::Char('g') => {
                            app.dialogbox.sel_dir = 0;

                            app.dialogbox.update_prev_dirs();
                        },

                        KeyCode::Char('h') => {
                            app.dialogbox.go_back_dir();
                            app.dialogbox.update_prev_dirs();
                        }

                        KeyCode::Char('l') => {
                            app.dialogbox.go_forward_dir();
                            app.dialogbox.update_prev_dirs();
                        }

                        _ => ()

                    }

                }


                // binds for preview contents / right
                if app.sel_window  == CurrentWindow::Right && app.preview_type == PreviewType::Contents {
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

                        KeyCode::Char('G') => app.sel_prev_conts_dir = app.preview_conts_dirs.len() - 1,
                        KeyCode::Char('g') => app.sel_prev_conts_dir = 0,

                        _ => ()
                    }
                }

                // binds for preview readme/todo / right
                if app.sel_window == CurrentWindow::Right && (app.preview_type == PreviewType::TODO || app.preview_type == PreviewType::README) {
                    let num =
                    match key.modifiers == KeyModifiers::SHIFT {
                        true => 5,
                        false => 1,
                    };
                    match key.code {
                        KeyCode::Char('H') | KeyCode::Left
                            => app.preview_scroll.1 = app.preview_scroll.1.saturating_sub(num),
                        KeyCode::Char('j') | KeyCode::Char('J') | KeyCode::Down
                            => app.preview_scroll.0 += num,
                        KeyCode::Char('k') | KeyCode::Char('K') | KeyCode::Up
                            => app.preview_scroll.0 = app.preview_scroll.0.saturating_sub(num),
                        KeyCode::Char('l') | KeyCode::Char('L') | KeyCode::Right
                            => app.preview_scroll.1 += num,
                        KeyCode::Char('h') 
                            => app.sel_window = CurrentWindow::Left,
                        
                        _ => ()
                    }
                }

                // binds for left / main window
                if let CurrentWindow::Left = app.sel_window {
                    match key.code {
                        KeyCode::Char('j') | KeyCode::Down => {
                            app.sel_dir = (app.sel_dir + 1) % app.dirs.len();

                            app.status_txt.clear();

                            app.read_todo_readme();
                            app.update_prev_dirs();
                        }

                        KeyCode::Char('k') | KeyCode::Up => {
                            app.sel_dir = match app.sel_dir {
                                0 => app.dirs.len() - 1,
                                _ => app.sel_dir - 1 
                            };

                            app.status_txt.clear();

                            app.read_todo_readme();
                            app.update_prev_dirs();
                        }

                        KeyCode::Char('l') | KeyCode::Right => {
                            if !app.preview_file_conts.is_empty() || app.preview_type == PreviewType::Contents {
                                app.sel_window = CurrentWindow::Right
                            }
                        },

                        KeyCode::Char('f') => {
                            std::env::set_current_dir(&app.dirs[app.sel_dir]).unwrap();
                            std::process::Command::new("git").arg("fetch").output().unwrap();
                        }

                        KeyCode::Char('p') => {
                            if crossterm::event::poll(std::time::Duration::from_millis(200))? {
                                if let Event::Key(key) = event::read()? {
                                    if key.code == KeyCode::Char('p') {
                                        std::env::set_current_dir(&app.dirs[app.sel_dir]).unwrap();
                                        let out = std::process::Command::new("git").arg("pull").output().unwrap();
                                        let stdout = out.stdout;
                                        let stderr = out.stderr;

                                        app.status_txt = 
                                            String::from_utf8(stdout).unwrap().replace("\n", " ") 
                                            + &String::from_utf8(stderr).unwrap().replace("\n", " ");

                                        app.update_prev_dirs();
                                    }
                                }
                            }
                        }

                        KeyCode::Char('D') => {
                            app.undo_vec.push(
                                (
                                    app.dirs.remove(app.sel_dir),
                                    app.sel_dir
                                )
                            );
                            if app.sel_dir == app.dirs.len() && app.sel_dir != 0 {
                                app.sel_dir -= 1;
                            }
                            app.update_prev_dirs();
                        }

                        KeyCode::Char('u') => {
                            if let Some(undo_dir) = app.undo_vec.pop(){
                                app.dirs.insert(undo_dir.1, undo_dir.0)
                            }
                            app.update_prev_dirs();
                        }

                        KeyCode::Char('G') => {
                            app.sel_dir = app.dirs.len() - 1;

                            app.update_prev_dirs();
                        },
                        KeyCode::Char('g') => {
                            app.sel_dir = 0;

                            app.update_prev_dirs();
                        },
                        

                        _ => ()
                    }
                }

            } // match block else

        } // match key.code
    } // is there key event
  Ok(())
}
