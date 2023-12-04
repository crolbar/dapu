use crossterm::event::{KeyCode, Event, self};
use crate::{app::{App, PreviewType, MainWindows}, tui::Tui};
use std::io::Result;


pub fn update(app: &mut App, _tui: &mut Tui) -> Result<()> {
  if let Event::Key(key) = event::read()? {
      match key.code {
        KeyCode::Char('q') => {app.exit = true; }

        KeyCode::Char('j') => {
            app.sel_dir = (app.sel_dir + 1) % app.dirs.len()
        }
        KeyCode::Char('k') => {
            app.sel_dir = match app.sel_dir {
                0 => app.dirs.len() - 1,
                _ => app.sel_dir - 1 
            }
        }

        KeyCode::Char('h') => {
            app.sel_window = MainWindows::Right 
        }
        KeyCode::Char('l') => {
            app.sel_window = MainWindows::Left
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

        KeyCode::Enter => {
            match app.sel_window {
                MainWindows::Left =>  {
                    app.save_to_conf();
                    app.exit = true;

                    let path = &app.dirs[app.sel_dir];

                    if app.only_output_path {
                        println!("{}", path.to_str().unwrap());
                    } else {
                        std::process::Command::new("nvim").arg(path).status().unwrap();
                    }

                }
                MainWindows::Right =>  {

                }
            }
        }

        _ => (),
    }
  }

  Ok(())
}
