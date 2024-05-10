use crossterm::event::{KeyCode, Event, self, KeyModifiers};
use crate::{app::{App, PreviewType, CurrentWindow}, tui::Tui};
use std::io::Result;


pub fn update(app: &mut App, _tui: &mut Tui) -> Result<()> {
    if let Event::Key(key) = event::read()? {
        if app.seach.is_typing {
            if let KeyCode::Char(char) = key.code {
                    app.seach.txt.push(char);

                if app.is_focused_dialog() {
                    app.seach.seach_from_dirs(&mut app.dialogbox.dirs);

                    if app.dialogbox.sel_dir >= app.dialogbox.dirs.len() {
                        app.dialogbox.sel_dir = 0;
                    }
                    if !app.dialogbox.dirs.is_empty() {
                        app.dialogbox.update_prev_dirs();
                    }
                } else {
                    app.seach.seach_from_dirs(&mut app.dirs);

                    if app.sel_dir >= app.dirs.len() {
                        app.sel_dir = 0;
                    }
                    if !app.dirs.is_empty() {
                        app.update_right_pane()
                    }
                }
            }

            match key.code {
                KeyCode::Backspace => {
                    let dirs = if app.is_focused_left() {
                        &mut app.dirs
                    } else {
                        &mut app.dialogbox.dirs
                    };

                    app.seach.txt.pop();
                    app.seach.seach_from_dirs(dirs);

                    if app.is_focused_dialog() {
                        if app.dialogbox.sel_dir >= app.dialogbox.dirs.len() {
                            app.dialogbox.sel_dir = 0
                        }
                        if !app.dialogbox.dirs.is_empty() {
                            app.dialogbox.update_prev_dirs();
                        }
                    } else {
                        if app.sel_dir > app.dirs.len() {
                            app.sel_dir = 0;
                        }
                        if !app.dirs.is_empty() {
                            app.update_right_pane()
                        }
                    }
                },
                KeyCode::Esc | KeyCode::Enter => {
                    if (app.dirs.len() == app.seach.main_dirs.len() || app.dirs.is_empty()) && app.is_focused_left() {
                        app.seach.revert_dirs(&mut app.dirs)
                    } 

                    if app.is_focused_dialog() {
                        if app.dialogbox.dirs.is_empty() {
                            app.seach.revert_dirs(&mut app.dialogbox.dirs)
                        } else {
                            app.seach.main_dirs.clear();
                        }
                    }
                    app.seach.exit()
                },

                _ => ()
            }
        } else {

        // binds for all windows exept when typing
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => {
                    if app.seach.main_dirs.is_empty() {
                        app.save_to_conf();
                        app.exit = true;
                    } else {
                        app.seach.revert_dirs(&mut app.dirs);

                        app.update_right_pane()
                    }
                }
                KeyCode::Char('a') => {
                    if app.seach.main_dirs.is_empty() {
                        if !app.is_focused_dialog() {
                            app.dialogbox.open_home_dir();
                            app.dialogbox.update_prev_dirs();
                            app.set_focus_dialog();
                        } else {
                            app.seach.main_dirs.clear();
                            app.set_focus_left()
                        }
                    }
                },

                KeyCode::Char('o') => app.only_output_path = !app.only_output_path,

                KeyCode::Char('C') => {
                    app.preview_type = PreviewType::Contents;
                    app.update_right_pane()
                },
                KeyCode::Char('T') => {
                    app.preview_type = PreviewType::TODO;
                    app.update_right_pane()
                },
                KeyCode::Char('R') => {
                    app.preview_type = PreviewType::README;
                    app.update_right_pane()
                },


                KeyCode::Enter | KeyCode::Char(' ') => {
                    enter_fn(key, app);

                    // if the dir was selected from search
                    if !app.seach.main_dirs.is_empty() {
                        app.seach.set_sel_dir(&mut app.sel_dir, &app.dirs);
                        app.seach.revert_dirs(&mut app.dirs)
                    }

                    app.save_to_conf();
                }

                _ => {
                    // binds for dialog box 
                    if app.is_focused_dialog() {
                        match key.code {
                            KeyCode::Char('j') | KeyCode::Down => {
                                app.dialogbox.sel_dir = (app.dialogbox.sel_dir + 1) % app.dialogbox.dirs.len();

                                app.dialogbox.update_prev_dirs();
                            }

                            KeyCode::Char('k') | KeyCode::Up => {
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

                            KeyCode::Char('h') | KeyCode::Left | KeyCode::Char('H') => {
                                app.dialogbox.go_back_dir();
                                app.dialogbox.update_prev_dirs();
                                app.seach.main_dirs.clear();
                            }

                            KeyCode::Char('l') | KeyCode::Right => {
                                app.dialogbox.go_forward_dir();
                                app.dialogbox.update_prev_dirs();
                                app.seach.main_dirs.clear();
                            }

                            KeyCode::Char('D') => {
                                if let Some(dir) = app.dirs.iter().position(|d| d == &app.dialogbox.dirs[app.dialogbox.sel_dir]) {
                                    app.dirs.remove(dir);
                                }
                                if app.sel_dir >= app.dirs.len() {
                                    app.sel_dir = app.dirs.len().saturating_sub(1)
                                }
                            }

                            KeyCode::Char('/') => {
                                if app.seach.main_dirs.is_empty(){
                                    app.seach.is_typing = true
                                }
                            },

                            _ => ()
                        }
                    }


                    // binds for preview contents / right
                    if app.is_focused_right() && app.preview_type == PreviewType::Contents {
                        match key.code {
                            KeyCode::Char('j') | KeyCode::Down => {
                                app.prev.sel_dir = (app.prev.sel_dir + 1) % app.prev.dirs.len()
                            }

                            KeyCode::Char('k') | KeyCode::Up => {
                                app.prev.sel_dir = match app.prev.sel_dir {
                                    0 => app.prev.dirs.len() - 1,
                                    _ => app.prev.sel_dir - 1 
                                }
                            }
                            KeyCode::Char('{') => {
                                if app.prev.sel_dir == 0 || app.prev.sel_dir as i32 - 3 < 0 {
                                    app.prev.sel_dir = app.prev.dirs.len() - 1;
                                } else {
                                    app.prev.sel_dir -= 3;
                                }
                            }
                            KeyCode::Char('}') => {
                                if app.prev.sel_dir + 3 > app.prev.dirs.len() {
                                    app.prev.sel_dir = 0;
                                } else {
                                    app.prev.sel_dir += 3;
                                }
                            }

                            KeyCode::Char('h') | KeyCode::Left => app.set_focus_left(),

                            KeyCode::Char('G') => app.prev.sel_dir = app.prev.dirs.len() - 1,
                            KeyCode::Char('g') => app.prev.sel_dir = 0,

                            _ => ()
                        }
                    }

                    // binds for preview readme/todo / right
                    if app.is_focused_right() && app.preview_type != PreviewType::Contents {
                        let num =
                            match key.modifiers == KeyModifiers::SHIFT {
                                true => 5,
                                false => 1,
                            };
                        match key.code {
                            KeyCode::Char('H') | KeyCode::Left
                                => app.prev.scroll.1 = app.prev.scroll.1.saturating_sub(num),
                            KeyCode::Char('j') | KeyCode::Char('J') | KeyCode::Down
                                => app.prev.scroll.0 += num,
                            KeyCode::Char('k') | KeyCode::Char('K') | KeyCode::Up
                                => app.prev.scroll.0 = app.prev.scroll.0.saturating_sub(num),
                            KeyCode::Char('l') | KeyCode::Char('L') | KeyCode::Right
                                => app.prev.scroll.1 += num,
                            KeyCode::Char('g') => app.prev.scroll = (0,0),
                            KeyCode::Char('G') => app.prev.scroll = (app.prev.file_txt.matches("\n").count() as u16, 0),
                            KeyCode::Char('h') => app.set_focus_left(),

                                _ => ()
                        }
                    }

                    // binds for left / main window
                    if app.is_focused_left() {
                        match key.code {
                            KeyCode::Char('j') | KeyCode::Down => {
                                app.sel_dir = (app.sel_dir + 1) % app.dirs.len();

                                app.status_txt.clear();
                                app.update_right_pane()
                            }

                            KeyCode::Char('k') | KeyCode::Up => {
                                app.sel_dir = match app.sel_dir {
                                    0 => app.dirs.len() - 1,
                                    _ => app.sel_dir - 1 
                                };

                                app.status_txt.clear();
                                app.update_right_pane()
                            }

                            KeyCode::Char('l') | KeyCode::Right => {
                                if !app.prev.file_txt.is_empty() || !app.prev.dirs.is_empty() {
                                    app.set_focus_right()
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

                                            app.update_right_pane()
                                        }
                                    }
                                }
                            }

                            KeyCode::Char('D') => {
                                if app.dirs.len() != 1 {
                                    let removed_dir = (app.dirs.remove(app.sel_dir), app.sel_dir);

                                    app.redo_vec.clear();

                                    app.undo_vec.push(removed_dir);

                                    if app.sel_dir == app.dirs.len() && app.sel_dir != 0 {
                                        app.sel_dir -= 1;
                                    }
                                    app.update_right_pane();
                                    app.save_to_conf();
                                }
                            }

                            KeyCode::Char('u') => {
                                if let Some(undo_dir) = app.undo_vec.pop() {
                                    app.redo_vec.push(undo_dir.clone());

                                    if undo_dir.1 > app.dirs.len() {
                                        app.dirs.push(undo_dir.0);
                                    } else {
                                        app.dirs.insert(undo_dir.1, undo_dir.0);
                                    }
                                    app.update_right_pane();
                                    app.save_to_conf();
                                } else { app.status_txt = "Already at oldest change".to_string() }
                            }

                            KeyCode::Char('r') | KeyCode::Char('y') => { if key.modifiers == KeyModifiers::CONTROL {
                                if let Some(redo_dir) = app.redo_vec.pop() {
                                    app.undo_vec.push(redo_dir.clone());
                                    app.dirs.remove(app.dirs.iter().position(|d| d == &redo_dir.0).unwrap());
                                    app.update_right_pane();
                                    app.save_to_conf();
                                } else { app.status_txt = "Already at newest change".to_string() }
                            }}

                            KeyCode::Char('G') => {
                                app.sel_dir = app.dirs.len() - 1;
                                app.update_right_pane()
                            }
                            KeyCode::Char('g') => {
                                app.sel_dir = 0;
                                app.update_right_pane()
                            }
                            KeyCode::Char('{') => {
                                if app.sel_dir == 0 || app.sel_dir as i32 - 3 < 0 {
                                    app.sel_dir = app.dirs.len() - 1;
                                } else {
                                    app.sel_dir -= 3;
                                }

                                app.update_right_pane()
                            }
                            KeyCode::Char('}') => {
                                if app.sel_dir + 3 > app.dirs.len() {
                                    app.sel_dir = 0;
                                } else {
                                    app.sel_dir += 3;
                                }

                                app.update_right_pane()
                            }

                            KeyCode::Char('/') => {
                                if app.seach.main_dirs.is_empty(){
                                    app.seach.is_typing = true
                                }
                            },


                            _ => ()
                        }
                    }
                } // match block else
            } // match key.code
        } // is typing else
    } // is there key event
  Ok(())
}

fn enter_fn(key: crossterm::event::KeyEvent, app: &mut App) {
    app.exit = true;

    let path = 
        match app.sel_window {
            CurrentWindow::Right => &app.prev.dirs[app.prev.sel_dir],
            CurrentWindow::Left => &app.dirs[app.sel_dir],
            CurrentWindow::Dialog => &app.dialogbox.dirs[app.dialogbox.sel_dir],
        };

    // if in dialogbox add dir to main dirs vector
    if app.is_focused_dialog() {
        app.exit = false;
        if key.code == KeyCode::Char(' ') {
            if let Some(dir) = app.dirs.iter().position(|d| d == &app.dialogbox.dirs[app.dialogbox.sel_dir]) {
                app.dirs.remove(dir);
                if app.sel_dir >= app.dirs.len() {
                    app.sel_dir = app.dirs.len().saturating_sub(1)
                }
            } else 
            if !app.dirs.contains(path) {
                app.dirs.push(path.to_path_buf());
                app.save_to_conf();
            }
        } else {
            app.dialogbox.go_forward_dir();
            app.dialogbox.update_prev_dirs();
            app.seach.main_dirs.clear();
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
            true => println!("{}", path.to_str().unwrap()),
            false => { std::process::Command::new(&app.default_editor).arg(path).status().unwrap(); }
        }
    }
}
