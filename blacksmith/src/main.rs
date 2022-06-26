use std::{env, io, path::Path, process};

use clap::ArgMatches;
use mdbook::{
    errors::Error,
    preprocess::{CmdPreprocessor, Preprocessor},
};

use mdbook_blacksmith::Blacksmith;

const CACHE_FILE: &str = ".blacksmith-cache.json";
const CACHE_TTL_SECONDS: u64 = 3600; // 1 hour

fn main() {
    // If RUST_LOG is present use that, else default to info level printing.
    if let Ok(v) = env::var("RUST_LOG") {
        env_logger::init_from_env(v);
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
    )
    .get_matches();

    macro_rules! log_unwrap {
        ($result:expr) => {
            match $result {
                Ok(value) => value,
                Err(error) => {
                    log::error!("{}", error);
                    process::exit(1);
                }
            }
        };
    }

    let cache_file = Path::new(CACHE_FILE);
    let mut blacksmith = if cache_file.is_file() {
        log_unwrap!(serde_json::from_slice(&log_unwrap!(std::fs::read(
            &cache_file
        ))))
    } else {
        Blacksmith::new()
    };

    if let Some(sub_args) = matches.subcommand_matches("supports") {
        handle_supports(&blacksmith, sub_args);
    } else {
        let mut update_cache = false;
        if blacksmith.is_stale(CACHE_TTL_SECONDS) {
            blacksmith = log_unwrap!(Blacksmith::init());
            update_cache = true;
        } else {
            log::info!("Using cached data in {}", cache_file.display());
        }
        log_unwrap!(handle_preprocessing(&blacksmith));

        if update_cache {
            log::info!("Storing the cache in {}", cache_file.display());
            log_unwrap!(std::fs::write(
                &cache_file,
                &log_unwrap!(serde_json::to_vec(&blacksmith))
            ));
        }
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
