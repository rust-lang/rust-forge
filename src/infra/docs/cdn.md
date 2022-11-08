# Content Delivery Networks

Users of the Rust programming language interact with the infrastructure of the
project in various different ways. They access the project's website and
documentation, query the crates index, and download Rust releases and crates.
These resources are hosted by the Rust project and served through a [Content
Delivery Network] (CDN).

This document outlines why we use CDNs, for what, and how we have set them up.

## Objectives

- Reduce costs of outbound traffic by caching resources in the CDN
- Reduce load on origin servers to save compute resources
- Provide a way to rewrite legacy URLs for some resources

## Infrastructure

Most of the project's resources are hosted on [AWS]. Static content is stored in
[Amazon S3], while dynamic content is loaded from a server. Both types of
content are served through [Amazon CloudFront], the [Content Delivery Network]
of AWS.

When a user access a resource, e.g. they are trying to download a crate, they
will access the resource through the CDN. Different _distributions_ map domain
names to a configuration and a backend (called the _origin_). For example,
downloading a crate from `static.crates.io` goes through a _distribution_ that
fetches the crate from an S3 bucket and then caches it for future requests.

```text
                             ┌──► S3 (static content)
                             │
User ───────► CloudFront ────┤
                             │
                             └──► Server (dynamic content)
```

## Distributions

There are many distributions, all of which are configured in the
[rust-lang/simpleinfra] repository. However, their usage is very unevenly
distributed. The following distributions are the most important ones for the
project, both in terms of traffic and criticality for the ecosystem.

### Rust Releases

Whenever a user installs or updates Rust, pre-compiled binaries are downloaded
from `static.rust-lang.org`. The same is true when Rust is installed in a CI/CD
pipeline, which is why this distribution has by far the highest traffic volume.

Rust binaries are static and are stored in [Amazon S3], from where they are
served by the CloudFront distribution.

The distribution for `static.rust-lang.org` has a custom router that runs in a
[AWS Lambda] function. The router provides a way to list files for a release and
rewrites the legacy URL for `rustup.sh`.

The cache for Rust releases is invalidated nightly.

### Crates

Similar to Rust releases, crates are served from as static content from
`static.crates.io`. While still being the second-largest distribution in our
infrastructure, it is much smaller than the releases.

Crates are static and stored in [Amazon S3], and served through a CloudFront
distribution.

[amazon cloudfront]: https://aws.amazon.com/cloudfront/
[amazon s3]: https://aws.amazon.com/s3/
[aws]: https://aws.amazon.com/
[aws lambda]: https://aws.amazon.com/lambda/
[content delivery network]: https://en.wikipedia.org/wiki/Content_delivery_network
[rust-lang/simpleinfra]: https://github.com/rust-lang/simpleinfra