use std::{env, io, process};

use clap::{clap_app, ArgMatches};
use mdbook::{errors::Error, preprocess::{CmdPreprocessor, Preprocessor}};

use mdbook_blacksmith::Blacksmith;

fn main() {
    // If RUST_LOG is present use that, else default to info level printing.
    if env::var("RUST_LOG").is_ok() {
        env_logger::init();
    } else {
        env_logger::builder()
            .filter_level(log::LevelFilter::Info)
            .init();
    }

    let matches = clap::clap_app!(blacksmith =>
        (about: clap::crate_description!())
        (@subcommand supports =>
            (about: "Check whether a renderer is supported by this preprocessor")
            (@arg renderer: +takes_value +required)
        )
    ).get_matches();

    macro_rules! log_unwrap {
        ($result:expr) => {
            match $result {
                Ok(value) => value,
                Err(error) => {
                    log::error!("{}", error);
                    process::exit(1);
                }
            }
        }
    }

    let blacksmith = Blacksmith::new();

    if let Some(sub_args) = matches.subcommand_matches("supports") {
        handle_supports(&blacksmith, sub_args);
    } else  {
        log_unwrap!(handle_preprocessing(&log_unwrap!(blacksmith.init())))
    }
}

fn handle_preprocessing(pre: &Blacksmith) -> Result<(), Error> {
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    if ctx.mdbook_version != mdbook::MDBOOK_VERSION {
        // We should probably use the `semver` crate to check compatibility
        // here...
        log::warn!(
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

fn handle_supports(pre: &Blacksmith, sub_args: &ArgMatches) -> ! {
    let renderer = sub_args.value_of("renderer").expect("Required argument");
    let supported = pre.supports_renderer(&renderer);

    // Signal whether the renderer is supported by exiting with 1 or 0.
    if supported {
        process::exit(0);
    } else {
        process::exit(1);
    }
}

