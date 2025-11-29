mod channel;

use std::{collections::BTreeMap, fmt::Write, time::SystemTime};

use mdbook::{
    book::{Book, BookItem},
    errors::Error,
    preprocess::{Preprocessor, PreprocessorContext},
};

const CHANNELS: &[&str] = &["stable", "beta", "nightly"];
const CHANNEL_URL_PREFIX: &str = "https://static.rust-lang.org/dist/channel-rust-";
const MANIFESTS_URL: &str = "https://static.rust-lang.org/manifests.txt";

/// A representation of a rust target platform. `stable`, `beta`, and `nightly`
/// represent whether the platform is available on that channel. `stable` also
/// carries the specific stable compiler version.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Platform {
    stable: Option<String>,
    beta: bool,
    nightly: bool,
}

impl Default for Platform {
    fn default() -> Self {
        Self {
            stable: None,
            beta: false,
            nightly: false,
        }
    }
}

/// `Blacksmith` builds dynamic tables and lists to display in the Rust Forge.
#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct Blacksmith {
    last_update: Option<u64>,
    stable_version: Option<String>,
    platforms: BTreeMap<String, Platform>,
    #[serde(default)]
    previous_stable_versions: Vec<(String, Vec<String>)>,
}

struct SemVer {
    minor: u32,
}

impl From<&String> for SemVer {
    fn from(s: &String) -> Self {
        SemVer {
            minor: s
                .split(".")
                .nth(1)
                .unwrap()
                .parse()
                .expect("Invalid version string"),
        }
    }
}

impl Blacksmith {
    /// Creates a new instance of `Blacksmith`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if the data in this `Blacksmith` instance was not updated within the TTL.
    pub fn is_stale(&self, ttl: u64) -> bool {
        if let (Some(last_update), Some(now)) = (self.last_update, unix_time()) {
            last_update + ttl < now
        } else {
            true
        }
    }

    /// Populates a `Blacksmith` instance with data gathered from Rust's CI and
    /// distribution channels.
    pub fn init() -> Result<Self, Box<dyn std::error::Error>> {
        let mut blacksmith = Self::new();

        for &channel_name in CHANNELS {
            let channel_url = format!("{}{}.toml", CHANNEL_URL_PREFIX, channel_name);
            let content = reqwest::blocking::get(&channel_url)?.text()?;
            let rust = toml::from_str::<crate::channel::Channel>(&content)?
                .pkg
                .rust;
            log::info!(
                "Found {} targets for {} channel (v{})",
                rust.target.len(),
                channel_name,
                rust.version
            );

            let vers = rust.version.split(' ').next().unwrap().to_string();
            let platforms = rust
                .target
                .into_iter()
                .filter_map(|(target, content)| {
                    if content.available {
                        Some(target)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            if channel_name == "stable" {
                blacksmith.stable_version = Some(vers.clone());
            }

            for platform in platforms {
                let entry = blacksmith
                    .platforms
                    .entry(platform)
                    .or_insert_with(Platform::default);

                match channel_name {
                    "stable" => entry.stable = Some(vers.clone()),
                    "beta" => entry.beta = true,
                    "nightly" => entry.nightly = true,
                    _ => unreachable!(),
                }
            }
        }

        let latest_stable_version = &blacksmith.stable_version.clone().unwrap();

        // We need to use a map to deduplicate entries and sort them in the correct order.
        // There are multiple entries for stable 1.8.0, 1.14.0, 1.15.1, 1.49.0 in MANIFESTS_URL.
        // Keys contain (minor_version, patch_version) and values contain (full_version, platforms).
        let mut previous_stable_version_map: BTreeMap<(u32, u32), (String, Vec<String>)> =
            BTreeMap::new();

        // Go over stable versions in https://static.rust-lang.org/manifests.txt in reverse order.
        let manifests_content = reqwest::blocking::get(MANIFESTS_URL)?.text()?;
        let stable_manifest_url_regex = regex::Regex::new(
            r"^static\.rust-lang\.org/dist/\d{4}-\d{2}-\d{2}/channel-rust-1\.(\d+)\.(\d+)\.toml$",
        )
        .unwrap();
        for manifest_url in manifests_content.lines().rev() {
            let minor;
            let patch;

            // Check if it's a stable version.
            if let Some(captures) = stable_manifest_url_regex.captures(&(manifest_url)) {
                minor = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
                patch = captures.get(2).unwrap().as_str().parse::<u32>().unwrap();
            } else {
                continue;
            }

            let full_version = format!("1.{}.{}", minor, patch);

            // Check if we already processed that version.
            if previous_stable_version_map.contains_key(&(minor, patch)) {
                continue;
            }

            // Skip latest stable version.
            if &full_version == latest_stable_version {
                continue;
            }

            // Download https://static.rust-lang.org/dist/channel-rust-{major.minor.patch}.toml and process it.
            let channel_url = format!("{}{}.toml", CHANNEL_URL_PREFIX, full_version);

            let content = reqwest::blocking::get(&channel_url)?.text()?;
            let rust = toml::from_str::<crate::channel::Channel>(&content)?
                .pkg
                .rust;

            log::info!(
                "Found {} targets for stable v{}",
                rust.target.len(),
                rust.version
            );

            let version = rust.version.split(' ').next().unwrap().to_string();

            // Sanity check.
            assert_eq!(&full_version, &version);

            let platforms = rust
                .target
                .into_iter()
                .filter_map(|(target, content)| {
                    if content.available {
                        Some(target)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            previous_stable_version_map.insert((minor, patch), (version, platforms));
        }

        // There are no manifests for stable versions before 1.8.0,
        // so we hardcode them instead.
        // i686-pc-windows-msvc wasn't supported until version 1.3.0.
        for minor in 3..8 {
            previous_stable_version_map.insert(
                (minor, 0),
                (
                    format!("1.{}.0", minor),
                    Vec::from([
                        "i686-apple-darwin".to_string(),
                        "i686-pc-windows-gnu".to_string(),
                        "i686-pc-windows-msvc".to_string(),
                        "i686-unknown-linux-gnu".to_string(),
                        "x86_64-apple-darwin".to_string(),
                        "x86_64-pc-windows-gnu".to_string(),
                        "x86_64-pc-windows-msvc".to_string(),
                        "x86_64-unknown-linux-gnu".to_string(),
                    ]),
                ),
            );
        }

        // x86_64-pc-windows-msvc wasn't supported until version 1.2.0.
        previous_stable_version_map.insert(
            (2, 0),
            (
                "1.2.0".to_string(),
                Vec::from([
                    "i686-apple-darwin".to_string(),
                    "i686-pc-windows-gnu".to_string(),
                    "i686-unknown-linux-gnu".to_string(),
                    "x86_64-apple-darwin".to_string(),
                    "x86_64-pc-windows-gnu".to_string(),
                    "x86_64-pc-windows-msvc".to_string(),
                    "x86_64-unknown-linux-gnu".to_string(),
                ]),
            ),
        );

        for minor in 0..2 {
            previous_stable_version_map.insert(
                (minor, 0),
                (
                    format!("1.{}.0", minor),
                    Vec::from([
                        "i686-apple-darwin".to_string(),
                        "i686-pc-windows-gnu".to_string(),
                        "i686-unknown-linux-gnu".to_string(),
                        "x86_64-apple-darwin".to_string(),
                        "x86_64-pc-windows-gnu".to_string(),
                        "x86_64-unknown-linux-gnu".to_string(),
                    ]),
                ),
            );
        }

        for (_, (version, platforms)) in previous_stable_version_map.into_iter().rev() {
            blacksmith
                .previous_stable_versions
                .push((version, platforms));
        }

        blacksmith.last_update = unix_time();
        Ok(blacksmith)
    }

    /// Generates a table of links to the standalone installer packages for
    /// each platform.
    fn generate_standalone_installers_table(&self) -> String {
        let mut buffer = String::new();
        writeln!(
            buffer,
            "platform | stable ({}) | beta | nightly",
            self.stable_version.as_ref().unwrap()
        )
        .unwrap();

        writeln!(buffer, "---------|--------|------|--------").unwrap();

        for (name, platform) in &self.platforms {
            let extensions: &[&str] = if name.contains("windows")
                && (!name.contains("gnullvm")
                    || SemVer::from(self.stable_version.as_ref().unwrap()).minor >= 93)
            {
                &["msi", "tar.xz"]
            } else if name.contains("darwin") {
                &["pkg", "tar.xz"]
            } else {
                &["tar.xz"]
            };

            let stable_links = platform
                .stable
                .as_ref()
                .map(|version| generate_standalone_links("rust", version, name, extensions))
                .unwrap_or_else(String::new);

            let beta_links = if platform.beta {
                generate_standalone_links("rust", "beta", name, extensions)
            } else {
                String::new()
            };

            let nightly_links = if platform.nightly {
                generate_standalone_links("rust", "nightly", name, extensions)
            } else {
                String::new()
            };

            writeln!(
                buffer,
                "`{name}` | {stable} | {beta} | {nightly}",
                name = name,
                stable = stable_links,
                beta = beta_links,
                nightly = nightly_links
            )
            .unwrap();
        }

        buffer
    }

    /// Generates tables of links to the previous stable standalone installer packages for
    /// each platform.
    fn generate_previous_stable_standalone_installers_tables(&self) -> String {
        let mut buffer = String::new();

        for (stable_version, platforms) in &self.previous_stable_versions {
            writeln!(buffer, "## Stable ({})", stable_version).unwrap();
            writeln!(buffer, "").unwrap();

            writeln!(buffer, "platform | stable ({})", stable_version).unwrap();
            writeln!(buffer, "---------|--------").unwrap();

            for name in platforms {
                let extensions: &[&str] = if name.contains("windows")
                    && (!name.contains("gnullvm") || SemVer::from(stable_version).minor >= 93)
                {
                    &["msi", "tar.gz"]
                } else if name.contains("darwin") {
                    &["pkg", "tar.gz"]
                } else {
                    &["tar.gz"]
                };

                let stable_links =
                    generate_standalone_links("rust", stable_version, name, extensions);

                writeln!(
                    buffer,
                    "`{name}` | {stable}",
                    name = name,
                    stable = stable_links,
                )
                .unwrap();
            }

            writeln!(buffer, "").unwrap();
        }

        buffer
    }

    /// Generates a similar table to `generate_standalone_installers_table`
    /// except for the rust source code packages.
    fn generate_source_code_table(&self) -> String {
        let mut buffer = String::new();

        writeln!(buffer, "Channel | Archives + Signatures").unwrap();
        writeln!(buffer, "--------|----------------------").unwrap();

        for &channel in CHANNELS {
            if channel == "stable" {
                let stable_version = self.stable_version.as_ref().unwrap();
                writeln!(
                    buffer,
                    "stable ({}) | {}",
                    stable_version,
                    generate_standalone_links("rustc", stable_version, "src", &["tar.xz"])
                )
                .unwrap();
            } else {
                writeln!(
                    buffer,
                    "{} | {}",
                    channel,
                    generate_standalone_links("rustc", &channel, "src", &["tar.xz"])
                )
                .unwrap();
            }
        }

        buffer
    }
}

/// Generates links to standalone installers provided a rust version or channel,
/// target name, and file extension.
fn generate_standalone_links(base: &str, stem: &str, name: &str, extensions: &[&str]) -> String {
    let mut result = String::new();
    for extension in extensions {
        if !result.is_empty() {
            result.push_str(" <br> ");
        }
        let url = format!(
            "https://static.rust-lang.org/dist/{base}-{stem}-{name}.{extension}",
            extension = extension,
            name = name,
            stem = stem,
            base = base,
        );

        result.push_str(&format!(
            "[{extension}]({url}) <br> [{extension}.asc]({url}.asc)",
            extension = extension,
            url = url,
        ));
    }
    result
}

fn unix_time() -> Option<u64> {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .ok()
        .map(|d| d.as_secs())
}

impl Preprocessor for Blacksmith {
    fn name(&self) -> &str {
        "blacksmith"
    }

    fn run(&self, _: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        fn recursive_replace(book_item: &mut BookItem, old: &str, new: &str) {
            let chapter = match book_item {
                BookItem::Chapter(chapter) => chapter,
                _ => return,
            };

            chapter.content = chapter.content.replace(old, new);

            for sub_item in &mut chapter.sub_items {
                recursive_replace(sub_item, old, new);
            }
        }

        let standalone_installers_table = self.generate_standalone_installers_table();
        let previous_stable_standalone_installers_tables =
            self.generate_previous_stable_standalone_installers_tables();
        let source_code_table = self.generate_source_code_table();

        // TODO: Currently we're performing a global search for any of the
        // variables as that's the most flexible for adding more dynamic
        // content, and the time to traverse is fast enough to not be noticeable.
        // However if the processing time begins to become a bottleneck this
        // should change.
        for item in &mut book.sections {
            recursive_replace(item, "{{#installer_table}}", &standalone_installers_table);
            recursive_replace(
                item,
                "{{#previous_stable_standalone_installers_tables}}",
                &previous_stable_standalone_installers_tables,
            );
            recursive_replace(item, "{{#source_code_table}}", &source_code_table);
        }

        Ok(book)
    }

    /// `Blacksmith`'s operations are renderer independent as they output
    /// markdown.
    fn supports_renderer(&self, _renderer: &str) -> bool {
        true
    }
}
