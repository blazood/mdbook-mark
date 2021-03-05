#[macro_use]
extern crate clap;

use chrono::Local;
use env_logger::Builder;
use mdbook_mark::{MarkPreprocessor, handle_supports, handle_preprocessor};
use std::{env, process};
use log::LevelFilter;
use clap::{App, AppSettings, SubCommand, Arg};
use std::io::Write;

const VERSION: &str = concat!("v", crate_version!());

fn main() {
    init_logger();
    let app = App::new(crate_name!())
        .about(crate_description!())
        .author("blazh <blazh@163.com>")
        .version(VERSION)
        .setting(AppSettings::GlobalVersion)
        .setting(AppSettings::ColoredHelp)
        .subcommand(
            SubCommand::with_name("supports")
                .arg(Arg::with_name("renderer").required(true))
                .about("Check whether a renderer is supported by this preprocessor"),
        )
        .subcommand(
            SubCommand::with_name("install")
                .about("install search into your book.toml")
        )
        .subcommand(
            SubCommand::with_name("preprocessor")
                .about("preprocessor")
        );


    let matches = app.get_matches();

    let mark_preprocessor = MarkPreprocessor{};

    if let Some(sub_args) = matches.subcommand_matches("supports") {
        handle_supports(&mark_preprocessor, sub_args);
    }else {
        if let Err(e) = handle_preprocessor(&mark_preprocessor){
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}

fn init_logger() {
    let mut builder = Builder::new();

    builder.format(|formatter, record| {
        writeln!(
            formatter,
            "{} [{}] ({}): {}",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            record.level(),
            record.target(),
            record.args()
        )
    });

    if let Ok(var) = env::var("RUST_LOG") {
        builder.parse_filters(&var);
    } else {
        // if no RUST_LOG provided, default to logging at the Info level
        builder.filter(None, LevelFilter::Info);
        // Filter extraneous html5ever not-implemented messages
        builder.filter(Some("html5ever"), LevelFilter::Error);
    }

    builder.init();
}