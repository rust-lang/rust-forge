---
layout: default
title: Rust Platform Support &middot; The Rust Programming Language
---

# Rust Platform Support

The Rust compiler runs on, and compiles to, a great number of platforms, though
not all platforms are equally supported. Rust's support levels are organized
into three tiers, each with a different set of guarantees.

Platforms are identified by their "target triple" which is the string to inform
the compiler what kind of output should be produced. The columns below indicate
whether the corresponding component works on the specified platform.

{% for tier in site.tiers %}

## {{ tier[0] }}

{{ tier[1].description }}

<table>
<tr>
    <th>target</th>
    <th>std</th>
    <th>rustc</th>
    <th>cargo</th>
    <th>notes</th>
</tr>
{% for target in tier[1].platforms %}
<tr>
<td><code>{{ target.tuple }}</code></td>
<td>{{ target.std }}</td>
<td>{{ target.rustc }}</td>
<td>{{ target.cargo }}</td>
<td>{{ target.notes }}</td>
</tr>
{% endfor %}
</table>

{{ tier[1].footnotes }}

{% endfor %}

But those aren't the only platforms Rust can compile to! Those are the ones with
built-in target definitions and/or standard library support. When linking only
to the core library, Rust can also target "bare metal" in the x86, ARM, MIPS,
and PowerPC families, though it may require defining custom target
specifications to do so. All such scenarios are tier 3.
