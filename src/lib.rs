use log::info;
use mdbook::preprocess::{Preprocessor, PreprocessorContext, CmdPreprocessor};
use mdbook::book::Book;
use mdbook::errors::Error;
use clap::ArgMatches;
use std::{process, io};
use mdbook::BookItem;
use regex::Regex;

#[macro_use]
extern crate lazy_static;

lazy_static! {
 static ref RE : Regex= Regex::new(r"==(?P<c>\S+?)==").unwrap();
}

pub struct MarkPreprocessor {}

impl MarkPreprocessor {
    pub fn handle_each_item(book_item: &mut BookItem) {
        match book_item {
            BookItem::Chapter(chapter) => {
                chapter.content = RE.replace_all(&chapter.content, "<mark>$c</mark>").into_owned();
                for item in &mut chapter.sub_items {
                    MarkPreprocessor::handle_each_item(item);
                }
            }
            _ => {}
        }
    }
}

impl Preprocessor for MarkPreprocessor {
    fn name(&self) -> &str {
        "mark"
    }

    fn run(&self, _: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        let ii = &mut book.sections;
        for section in ii {
            MarkPreprocessor::handle_each_item(section);
        }
        Ok(book)
    }

    fn supports_renderer(&self, _renderer: &str) -> bool {
        _renderer == "html"
    }
}

pub fn handle_preprocessor(pre: &dyn Preprocessor) -> Result<(), Error> {
    info!("mark start");
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    if ctx.mdbook_version != mdbook::MDBOOK_VERSION {
        // We should probably use the `semver` crate to check compatibility
        // here...
        eprintln!(
            "Warning: The {} plugin was built against version {} of mdbook, \
             but we're being called from version {}",
            pre.name(),
            mdbook::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed_book = pre.run(&ctx, book)?;

    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}

pub fn handle_supports(pre: &dyn Preprocessor, sub_args: &ArgMatches) -> ! {
    let renderer = sub_args.value_of("renderer").expect("Required argument");
    let supported = pre.supports_renderer(&renderer);

    // Signal whether the renderer is supported by exiting with 1 or 0.
    if supported {
        process::exit(0);
    } else {
        process::exit(1);
    }
}