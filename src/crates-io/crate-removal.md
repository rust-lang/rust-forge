# Crate removal procedure

If we get a DMCA takedown notice, here's what needs to happen:

## Contact Mozilla Legal

Before removing the crates, [get in touch with legal](../core/legal.md) and ask
an opinion from them on the received request and whether we have to comply with
it.

## Remove relevant version(s) and/or entire crates from crates.io

* Remove it from the database:

      heroku run -- cargo run --bin delete-crate [crate-name]

  or

      heroku run -- cargo run --bin delete-version [crate-name] [version-number]

* Remove the crate or version from the index. To remove an entire crate, remove
  the entire crate file. For a version, remove the line corresponding to the
  relevant version.

* Remove the crate archive(s) and readme file(s) from S3.

* Invalidate the CloudFront cache – remove both the relevant readme and crate
  files. If in doubt, invalidate `/*` to flush everything.

## Remove entire crates from docs.rs

The docs.rs application supports deleting all the documentation ever published
of a crate, by running a CLI command. The people who currently have permissions
to access the server and run it are:

* docs.rs Team:
  * [@QuietMisdreavus](https://github.com/QuietMisdreavus)
  * [@pietroalbini](https://github.com/pietroalbini)
  * [@onur](https://github.com/onur)
  * [@jyn514](https://github.com/jyn514)
* Infrastructure Team:
  * [@Mark-Simulacrum](https://github.com/Mark-Simulacrum)
* People with elevated 1password access

You can find the documentation on how to run the command [here][docsrs-howto].

[docsrs-howto]: https://forge.rust-lang.org/infra/docs/docs-rs.html#removing-a-crate-from-the-website
