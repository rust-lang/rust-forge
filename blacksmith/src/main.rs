use std::{io, path::Path, process};

use mdbook::{
    errors::Error,
    preprocess::{CmdPreprocessor, Preprocessor},
};

use mdbook_blacksmith::Blacksmith;

const CACHE_FILE: &str = ".blacksmith-cache.json";
const CACHE_TTL_SECONDS: u64 = 3600; // 1 hour

fn main() {
    // If RUST_LOG is present use that, else default to info level printing.
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

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

    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        Some("supports") => {
            let renderer = args.next().expect("renderer name must be second argument");
            handle_supports(&blacksmith, &renderer);
        }
        Some(arg) => {
            eprintln!("unknown argument: {arg}");
            std::process::exit(1);
        }
        None => {}
    }

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

fn handle_supports(pre: &Blacksmith, renderer: &str) -> ! {
    let supported = pre.supports_renderer(renderer);

    // Signal whether the renderer is supported by exiting with 1 or 0.
    if supported {
        process::exit(0);
    } else {
        process::exit(1);
    }
}
