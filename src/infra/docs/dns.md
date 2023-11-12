# Domain names and DNS

All the DNS records of the domains owned by the Rust Infrastructure team are
hosted on [AWS Route 53], and can be tweaked by members of the team. This
document contains instructions for them on how to make changes.

* [Changing DNS records of a domain managed with Terraform](#changing-dns-records-of-a-domain-managed-with-terraform)
* [Managing DNS for a new domain with Terraform](#managing-dns-for-a-new-domain-with-terraform)
* [Adding subdomain redirects](#adding-subdomain-redirects)
* [Transferring domain names to Rust](#transferring-domain-names-to-rust)

## Changing DNS records of a domain managed with Terraform

> **Warning:** not all domain names are yet managed with Terraform. In the
> [console][hosted-zones], if a zone's comment doesn't start with `[terraform]`
> you'll need to make changes manually from the UI. Work is underway to migrate
> every domain to Terraform though.

> **Warning:** [`terraform/services/dns`][dns-dir] only contains the definition
> of DNS records pointing to resources managed outside of Terraform. When
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

## Adding subdomain redirects

Our Terraform configuration supports creating redirects from an arbitrary
number of subdomains we control to an URL. Redirects are created with these
pieces of infrastructure:

* A S3 bucket for each set of redirects, named `rust-http-redirect-<HASH>`. The
  bucket has website hosting enabled, configured to redirect all the incoming
  requests to the chosen URL. This allows implementing redirects without an
  underlying server.

* An ACM certificate (plus the DNS records to validate it) for each set of
  redirects, with all the sources as alternate names. This is used to enable
  HTTPS redirects.

* A CloudFront distribution for each set of redirects to support HTTPS
  requests, using the previously generated ACM certificate and forwarding
  requests to the S3 bucket.

* Route53 records for each redirect in the related zones: CNAMEs
  for subdomains, and ALIASes for apex domains.

All the redirects are defined in [`terraform/redirects.tf`][redirects-file],
with a module for each destination URL. Either create a new module if you need
to redirect to a new URL, or add a new subdomain to an existing module. See an
example module here (take care of replacing the placeholders):

```terraform
module "redirect_<IDENTIFIER>" {
  source = "./modules/subdomain-redirect"
  providers = {
    aws       = "aws"
    aws.east1 = "aws.east1"
  }

  to = "<DESTINATION-URL>"
  from = {
    "<SUBDOMAIN-1>" = module.dns.zone_<DOMAIN-1-IDENTIFIER>,
    "<SUBDOMAIN-2>" = module.dns.zone_<DOMAIN-2-IDENTIFIER>,
  }
}
```

Once you made all the changes you can apply the configuration with:

```
terraform init
terraform apply
```

Note that each change is going to take around 15 minutes to deploy, as
CloudFront distribution changes are really slow to propagate. Also, it's normal
to see a bunch of resources being recreated when a domain is added or removed
from an existing redirect, as the ACM certificate will need to be regenerated.

## Transferring domain names to Rust

These are the steps a member of the infrastructure team needs to take to
transfer a domain name to the Rust project's registrar:

1. Ask inside the infrastructure team if this is a domain name the project
   wants to own. In some more complicated cases this will need to be escalated
   to the Leadership Council.

2. If the domain name doesn’t already use AWS Route 53 as its nameserver, ask
   the current owner of the domain a list of all the DNS records that will need
   to be migrated. Then, add all the records to a new hosted zone on Route 53
   before the transfer of the domain. See the [section below][transfer-dns] on
   transferring DNS for more information on this step.

3. Ask the current owner to unlock the domain name for transfer, and get the
   transfer code from them. The transfer code is key to transferring the
   domain, so avoid receiving it on public communication platforms.

4. Go to the [Transfer Domain][transfer] section of AWS Route 53 and enter the
   domain name. If it doesn’t give an error (which should detail which steps
   are missing) enter the transfer code you received earlier, and choose to use
   an existing Route 53 hosted zone (it should auto-complete the right one).
   Until the Rust Foundation is up, use Pietro’s details as the domain
   contacts. Finally review everything and complete the transfer process.

5. Tell the current owner to wait for an email from their registrar, which will
   ask to click on a link to confirm the domain name transfer.

6. The transfer process will take a while. Once admin@rust-lang.org receives an
   email telling the domain has been transferred you’re done! 🎉🎉🎉

[transfer-dns]: #transferring-dns
### Transferring DNS

Most domain names use their registrar as the DNS server, but that means that
once the domain is transferred away the old registrar also stops serving DNS
traffic. Because of that we need to ensure all the DNS records are correctly
copied over to AWS Route 53 before actually starting the transfer process.

Explicitly ask the current domain owner for all the `A`, `AAAA`, `CNAME`, `TXT`
and `MX` records. Everything except the `MX` records needs to be copied to the
[Terraform DNS configuration][dns-dir] (create a new file for the domain name,
and take inspiration from the other domain names).

If you notice some of the records are referring to HTTP redirect services
provided by the current registrar then those will have to wait until the domain
has been transferred. Once the transfer occured, add a new [domain
redirect][redirects-file] on Terraform. This has to be done after the transfer
to be able to request the TLS certificate for the HTTPS redirect.

If the domain has `MX` records those will need to be migrated to Mailgun. Go to
[Mailgun][mailgun] and add the domain name there. Ensure it’s in the US region,
it uses shared IPs, and it has a 1024 bit DKIM key (the 2048 keys do not fit
into a single AWS Route 53 record). Then copy all the records except the
`CNAME` tracking one over to the Terraform DNS configuration, and wait for the
domain to be transferred. Once the transfer happens go back to Mailgun and
verify the DNS settings for the domain. Finally, add the domain to the [team
repository’s `config.toml`][team-config] and create the mailing lists you need
through the usual process.


[AWS Route 53]: https://aws.amazon.com/route53/
[hosted-zones]: https://console.aws.amazon.com/route53/home#hosted-zones:
[dns-dir]: https://github.com/rust-lang/simpleinfra/tree/master/terraform/services/dns/
[outputs-file]: https://github.com/rust-lang/simpleinfra/blob/master/terraform/services/dns/outputs.tf
[redirects-file]: https://github.com/rust-lang/simpleinfra/blob/master/terraform/redirects.tf
[transfer]: https://console.aws.amazon.com/route53/home#DomainTransfer:
[mailgun]: https://app.mailgun.com
[team-config]: https://github.com/rust-lang/team/blob/master/config.toml
