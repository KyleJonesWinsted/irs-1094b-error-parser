#![warn(clippy::all, clippy::pedantic)]

mod ui;

use iced::{Application, Settings};
use irs_1094b_error_parser::{process_files, InputPaths};
use native_dialog::FileDialog;
use std::{env::args, process::exit};
use ui::{App, OPEN_FILE_ARG};

fn main() {
    if args().nth(1).unwrap_or_default() == OPEN_FILE_ARG {
        let path = FileDialog::new().show_open_single_file().unwrap().unwrap();
        print!("{:?}", path);
        exit(0);
    }
    if let Ok(paths) = InputPaths::get() {
        process_files(&paths).unwrap_or_else(|e| {
            eprintln!("{}", e);
            exit(1);
        });
        exit(0);
    }
    let settings = Settings::default();
    App::run(settings).unwrap();
}
