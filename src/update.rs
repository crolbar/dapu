use crossterm::event::{KeyCode, Event, self, KeyModifiers};
use crate::{app::{App, PreviewType, FloatWindows, CurrentWindow}, tui::Tui};
use std::io::Result;


pub fn update(app: &mut App, _tui: &mut Tui) -> Result<()> {
  if let Event::Key(key) = event::read()? {
      match key.code {
        KeyCode::Char('q') => app.exit = true,

        KeyCode::Char('o') => app.only_output_path = !app.only_output_path,

        KeyCode::Char('j') => {
            match app.sel_window {
                CurrentWindow::Right => {
                    app.sel_prev_conts_dir = (app.sel_prev_conts_dir + 1) % app.preview_conts_dirs.len()
                }
                CurrentWindow::Left => {
                    app.sel_dir = (app.sel_dir + 1) % app.dirs.len()
                }
                _ => ()
            }
        }

        KeyCode::Char('k') => {
            match app.sel_window {
                CurrentWindow::Right => {
                    app.sel_prev_conts_dir = match app.sel_prev_conts_dir {
                        0 => app.preview_conts_dirs.len() - 1,
                        _ => app.sel_prev_conts_dir - 1 
                    }
                }
                CurrentWindow::Left => {
                    app.sel_dir = match app.sel_dir {
                        0 => app.dirs.len() - 1,
                        _ => app.sel_dir - 1 
                    }
                }
                _ => ()
            }
        }

        KeyCode::Char('h') => {
            app.sel_window = CurrentWindow::Left
        }
        KeyCode::Char('l') => {
            app.sel_window = CurrentWindow::Right
        }

        KeyCode::Char('c') => {
            app.preview_type = PreviewType::Contents
        }
        KeyCode::Char('t') => {
            app.preview_type = PreviewType::TODO
        }
        KeyCode::Char('r') => {
            app.preview_type = PreviewType::README
        }

        KeyCode::Char('1') => app.sel_window = CurrentWindow::Float(FloatWindows::EditCustomComm),
        KeyCode::Char('2') => app.sel_window = CurrentWindow::Float(FloatWindows::ChangeEditor),
        KeyCode::Char('3') => app.sel_window = CurrentWindow::Float(FloatWindows::AddFolder),

        KeyCode::Enter => {
            app.save_to_conf();
            app.exit = true;

            let path = 
                match app.sel_window {
                    CurrentWindow::Right => &app.preview_conts_dirs[app.sel_prev_conts_dir],
                    CurrentWindow::Left => &app.dirs[app.sel_dir],
                    _ => unreachable!()
                };

            if key.modifiers == KeyModifiers::ALT {
                std::process::Command::new("nano").arg(path).status().unwrap();
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

        } // enter

        _ => (),
    } // match key.code


  }

  Ok(())
}
