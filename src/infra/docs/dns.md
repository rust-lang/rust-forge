# Domain names and DNS

All the DNS records of the domains owned by the Rust Infrastructure team are
hosted on [AWS Route 53], and can be tweaked by members of the team. This
document contains instructions for them on how to make changes.

## Changing DNS records of a domain managed with Terraform

> **Warning:** not all domain names are yet managed with Terraform. In the
> [console][hosted-zones], if a zone's comment doesn't start with `[terraform]`
> you'll need to make changes manually from the UI. Work is underway to migrate
> every domain to Terraform though.

> **Warning:** [`terraform/services/dns`][dns-dir] only contains the definition
> of domain names pointing to resources managed outside of Terraform. When
> Terraform manages a resource it will automatically add the required records
> on its own. See the service's documentation to learn where its Terraform
> configuration lives.

DNS records are managed in the [`terraform/services/dns`][dns-dir] directory of
our Terraform configuration. A file named after the domain name, ending in
`.tf`, exists for each managed domain, and it contains some basic information
plus its records.

The configuration supports adding A, CNAME, MX and TXT records. Inside the
module definition contained in the domain's file, each record type has its own
map: the map keys are the names of the records, while the values are a list of
record values.

For example, to add a `pages.rust-lang.org` CNAME pointing to
`rust-lang.github.io` you'll need to add this to
`terraform/services/dns/rust-lang.org`:

```terraform
module "rust_lang_org" {
  # ...

  CNAME = {
    "pages.rust-lang.org." = ["rust-lang.github.io"],
    # ...
  }
}
```

Once you made all the changes you can apply them with:

```
terraform apply
```

## Managing DNS for a new domain with Terraform

Setting up Terraform to manage the DNS records of a new domain name involves a
few steps. First of all you need to decide the identifier used inside
Terraform for that domain. By convention, the identifier is the domain name
itself with `.` and `-` replaced with `_`. For example `rust-lang.org` becomes
`rust_lang_org`.

Then you can create a file in [`terraform/services/dns`][dns-dir] named after
the domain name, ending in `.tf`, with this content (take care of replacing the
placeholders):

```terraform
module "<IDENTIFIER>" {
  source = "./domain"

  domain = "<DOMAIN-NAME>"
  comment = "<COMMENT-FOR-THE-DOMAIN>"
  ttl = 300
}
```

Finally you need to output the ID of the Route53 zone, allowing other parts of
our Terraform configuration to add records. Add this snippet to
[`terraform/services/dns/outputs.tf`][outputs-file]:

```terraform
# ...

output "zone_<IDENTIFIER>" {
  value = module.<IDENTIFIER>.zone_id
}
```

Once you're done you can apply the changes with:

```
terraform init
terraform apply
```

[AWS Route 53]: https://aws.amazon.com/route53/
[hosted-zones]: https://console.aws.amazon.com/route53/home#hosted-zones:
[dns-dir]: https://github.com/rust-lang/simpleinfra/tree/master/terraform/services/dns/
[outputs-file]: https://github.com/rust-lang/simpleinfra/blob/master/terraform/services/dns/outputs.tf
