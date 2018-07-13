---
layout: default
title: State Of Rust
---

This document is intended to document the current state of Rust's various
unstable features, their current status and what's required next to
be completed.

| Team | Feature | Status | What's next |
| ---- | ------- | ------ | ----------- |
| compiler | [RFC amendment #1384][30450] | Implemented | Blocked on promotion to error |
| compiler | [Specialization and lifetime dispatch][40582] | Unimplemented | Blocked on specialisation |
| compiler | [The "ptx-kernel" ABI][38788] | Implemented | Blocked on decision to stabilise |
| compiler | [The `#[wasm_custom_section]` attribute][51088] | Implemented | Blocked decision to change to a more general solution |
| compiler | [The `#[wasm_import_module]` attribute][52090] | Implemented | Blocked on documentation and small design consideration |
| compiler | [The `msp430-interrupt` calling convention/ABI][38487] | Implemented | Blocked on decision to stabilise |
| compiler | [The `thiscall` calling convention][42202] | Implemented | Blocked on decision to stabilise |
| compiler | [The `x86-interrupt` calling convention][40180] | Implemented | Blocked on LLVM bugs |
| compiler | [Trait bounds not checked on specializable associated types][33017] | Unimplemented | Blocked on implementation |
| compiler | [`#[may_dangle]`, a refined dropck escape hatch (RFC 1327)][34761] | Implemented | Blocked on decision to stabilise |
| compiler | [`arbitrary_self_types`][44874] | Implemented | Blocked on RFC |
| compiler | [`incoherent_fundamental_impls` compatibility lint][46205] | Implemented | Hard error PR blocked on second crater run |
| compiler | [`legacy_directory_ownership` future-compatibility warnings][37872] | Implemented | Blocked on making the lint deny by default |
| compiler | [`parenthesized_params_in_types_and_modules` future-compatibility warnings][42238] | Implemented | Blocked on making the lint a hard error |
| compiler | [`safe_packed_borrows` compatibility lint][46043] | Implemented | Blocked on making the lint hard error |
| compiler | [`underscore_literal_suffix` future-compatibility warnings][42326] | Implemented | Blocked on making the lint a hard error |
| compiler | [`where_clauses_object_safety` future compatibility lint][51443] | Implemented | Blocked on making the lint deny by default |
| compiler | [🔬 generic associated types (GAT)][44265] | Unimplemented | Blocked on implementation |
| dev-tools + T-rustdoc | [`rustdoc --display-warnings`][41574] | Implemented | Blocked on decision to stabilise |
| dev-tools | [Sanitizer support][39699] | Implemented | Blocked on decision to stabilise |
| dev-tools | [Stabilise `-Zsave-analysis`][43606] | Unimplemented | Blocked on whether a new approach supersedes this |
| dev-tools | [`#[link(kind)]` connecting libraries on Windows][37403] | Implemented | Blocked on name decision and stabilisation |
| dev-tools | [`--print target-spec-json`][38338] | Implemented | Blocked on decision to stabilise |
| dev-tools | [`-Z profile` ][42524] | Implemented | Blocked on decision to stabilise |
| dev-tools | [libtest JSON output][49359] | Implemented | Blocked on decision to stabilise |
| lang + T-libs | ["macro naming and modularisation" (RFC #1561)][35896] | Implementation in progress | Blocked on unresolved design decisions |
| lang + T-libs | [RFC 1566: Procedural macros][38356] | Implementation in progress | Blocked on macros 1.2 stabilisation |
| lang | ['Allow `Self` to appear in the where clause of trait impls'][38864] | Implemented | Blocked on [#35237] |
| lang | [128-bit integer support (RFC 1504)][35118] | Implementation in progress | Blocked on enums with 128 discriminant |
| lang | [Allocator traits and std::heap][32838] | Implementation in progress | Blocked on audit and design decisions |
| lang | [Allow all literals in attributes (Tracking Issue for RFC #1559)][34981] | Implemented | Blocked on decision to stabilise |
| lang | [Allowing overlapping implementations for marker trait.][29864] | Implemented | Blocked on documentation |
| lang | [Exclusive range patterns][37854] | Implemented | Blocked on decision to stabilise |
| lang | [Exhaustive integer patterns tracking issue][50907] | Implementation in progress | Blocked implementation being merged |
| lang | [Naked fns (RFC #1201)][32408] | Implemented | Blocked on design feedback and the inline assembly feature. |
| lang | [Option::xor][50512] | Implemented | Blocked on decision to stable |
| lang | [Patterns in functions without body][35203] | Implemented | Blocked on making lint deny by default |
| lang | [Promoting `!` to a type (RFC 1216)][35121] | Implementation in progress | Blocked on decision around coercion |
| lang | [RFC 1872: `exhaustive_patterns`][51085] | Implemented | Blocked on the never type |
| lang | [RFC 554: `pattern_parentheses` ][51087] | Implemented | Blocked on stabilisation PR |
| lang | [The `#[used]` attribute][40289] | Implemented | Blocked on decision to stabilise |
| lang | [The `lifetime` specifier for `macro_rules!`][34303] | Implemented | Waiting on 1.28.0 stable release |
| lang | [Tracking issue for `fundamental` feature][29635] | Unimplemented | Blocked on further dicussion |
| lang | [Tracking issue for `no_core` stabilization][29639] | Implemented | Blocked on lang items |
| lang | [Tracking issue: declarative macros 2.0][39412] | Implementation in progress | Blocked on macro 2.0 hygiene RFC |
| lang | [Trait aliases][41517] | Unimplemented | Blocked on implementation |
| lang | [Tweaks to object safety (RFC 2027)][43561] | Unimplemented | Blocked on implementation |
| lang | [Unsized tuple coercion][42877] | Implemented | Blocked on decision to stabilise |
| lang | [Untagged unions (RFC 1444)][32836] | Unimplemented | Blocked on unresolved questions. |
| lang | [`#[repr(packed = "N")]` (RFC 1399)][33158] | Unimplemented | Blocked on unresolved questions |
| lang | [`:vis` macro matcher][41022] | Implemented | Blocked on `crate` becoming a keyword |
| lang | [`?` macro repetition][48075] | Implemented | Blocked on decision around edition dependent behaviour |
| lang | [`?` operator and `catch` expressions (RFC 243, `question_mark`)][31436] | Implemented | Blocked on reservation of `try` operator. |
| lang | [`asm` (inline assembly)][29722] | Implemented | Blocked on unclear stability guarantees |
| lang | [`box_patterns`][29641] | Implemented | Blocked on `box_syntax` feature |
| lang | [`cfg_target_vendor`][29718] | Implemented | Blocked on decision for stabilisation |
| lang | [`custom_attribute` & `rustc_attrs`][29642] | Unimplemented | Blocked on decision for deprecation or stabilisation |
| lang | [`custom_derive`][29644] | Deprecated | Blocked on rocket for removal |
| lang | [`illegal_floating_point_literal_pattern` compatibility lint][41620] | Implemented | Blocked on making the lint deny by default |
| lang | [`impl Trait` (RFC 1522, RFC 1951, RFC 2071)][34511] | Implementation in progress | Blocked elision, error messages, and `abstract type` |
| lang | [`invalid_type_param_default` compatibility lint][36887] | Implemented | Blocked on making the lint a hard error |
| lang | [`legacy_constructor_visibility` compatibility lint][39207] | Implemented | Blocked on making the lint a hard error |
| lang | [`literal` fragment specifier (RFC 1576)][35625] | Implemented | Waiting on 1.28.0 stable release |
| lang | [`private_in_public` compatibility lint.][34537] | Implemented | Blocked on PR to make lint deny by default |
| lang | [`resolve_trait_on_defaulted_unit` compatibility lint][39216] | Implemented | Blocked on making the lint a hard error |
| lang | [`safe_extern_statics` compatibility lint][36247] | Implemented | Blocked on making the lint a hard error |
| lang | [`use $crate;` compatibility warning][37390] | Implemented | Blocked on making the lint deny by default |
| lang | [`{Range, RangeFrom, RangeTo}::contains`][32311] | Implemented | Blocked on [unresolved questions][range_questions] |
| lang | | [`associated_type_defaults`][29661] | Unimplemented | Blocked on decision on deprecation or semantic change  |
| libes | [Specialization (RFC 1210)][31844] | Implemented | Blocked on restrictions of lifetime dispatch |
| libs | [API convention for blocking-, timeout-, and/or deadline-related functions][46316] | Unimplemented | Blocked on FCP |
| libs | [Add `is_empty` function to `ExactSizeIterator`][35428] | Unimplemented | Blocked on unresolved design decisions |
| libs | [Adding more atomic integer types][32976] | Implementation in progress | Blocked on atomic `128` bit integers. |
| libs | [Allow a `HashMap` and `BTreeMap` entry to be replaced.][44286] | Implemented | Blocked on decision to stabilise |
| libs | [Cell::update][50186] | Implemented | Blocked on unresolved questions |
| libs | [Custom allocators in standard collections][42774] | Implementation in progress | Blocked on [#47043] and [#50882] |
| libs | [IP constructors][44582] | Implementation in progress | Blocked on moving the implementation to use associated consts |
| libs | [Integer methods for Wrapping][32463] | Implementation in progress | Some methods are still missing implementations.  |
| libs | [Path/PathBuf normalization methods][47402] | Unimplemented | Blocked on implementation |
| libs | [Pluggable panic implementations (RFC 1513)][32837] | Unimplemented | Blocked on unresolved design decisions |
| libs | [RFC 2045: improving `#[target_feature]`][44839] | Implemented | Blocked on documentation and stabilisation |
| libs | [Reversing the bit pattern in an integer][48763] | Implemented | Blocked on decision to stabilise |
| libs | [Tracking Issue: Duration::{as_nanos, as_micros, as_millis}][50202] | Implemented | Blocked on decision to stable |
| libs | [TrustedLen (`trusted_len`)][37572] | Implemented | Blocked on specialisation |
| libs | [TryFrom/TryInto traits][33417] | Implemented | Blocked on the `!`/never type |
| libs | [UnicodeVersion and UNICODE_VERSION][49726] | Implemented | Blocked on concerns on whether updating const values is a breaking change |
| libs | [Vec::remove_item][40062] | Implemented | Blocked on unresolved concerns |
| libs | [Vec::resize_default][41758] | Implemented | Blocked on decision to stabilise |
| libs | [`Box::into_raw_non_null`][47336] | Implemented | Blocked on decision to stabilise |
| libs | [`Read::initializer`][42788] | Implemented | Blocked on design decisions |
| libs | [`Result<Option>` and `Option<Result>` Conversion][47338] | Implemented | Blocked on unresolved questions |
| libs | [`ToOwned::clone_into`][41263] | Implemented | Blocked on concerns |
| libs | [`Vec::drain_filter` and `LinkedList::drain_filter`][43244] | Implemented | Blocked on design decisions |
| libs | [`exact_chunks,exact_chunks_mut`][47115] | Implemented | Blocked on unresolved questions |
| libs | [`ops::Try`][42327] | Implemented? | Unclear on what is required next |
| libs | [`ptr::offset_to`][41079] | Deprecated | Blocked on removal |
| libs | [`std::io::{set_panic, set_print}`][31343] | Implemented | Blocked on decision to stabilise |
| libs | [`step_trait` stabilization][42168] | Implementation in progress | Blocked on replacement methods and `TrustedLen` |
| libs | [`sync::Once` poisoning][33577] | Implemented | Blocked on inactivity |
| libs | [write_all_at/read_exact_at convenience methods ][51984] | Implemented | Blocked on decision to stabilise |


[29639]: https://github.com/rust-lang/rust/issues/29639
[29635]: https://github.com/rust-lang/rust/issues/29635
[29641]: https://github.com/rust-lang/rust/issues/29641
[29642]: https://github.com/rust-lang/rust/issues/29642
[29644]: https://github.com/rust-lang/rust/issues/29644
[29661]: https://github.com/rust-lang/rust/issues/29661
[29718]: https://github.com/rust-lang/rust/issues/29718
[29722]: https://github.com/rust-lang/rust/issues/29722
[29864]: https://github.com/rust-lang/rust/issues/29864
[30450]: https://github.com/rust-lang/rust/issues/30450
[31343]: https://github.com/rust-lang/rust/issues/31343
[31436]: https://github.com/rust-lang/rust/issues/31436
[31844]: https://github.com/rust-lang/rust/issues/31844
[32311]: https://github.com/rust-lang/rust/issues/32311
[32408]: https://github.com/rust-lang/rust/issues/32408
[32463]: https://github.com/rust-lang/rust/issues/32463
[32836]: https://github.com/rust-lang/rust/issues/32836
[32837]: https://github.com/rust-lang/rust/issues/32837
[32838]: https://github.com/rust-lang/rust/issues/32838
[32976]: https://github.com/rust-lang/rust/issues/32976
[33017]: https://github.com/rust-lang/rust/issues/33017
[33158]: https://github.com/rust-lang/rust/issues/33158
[33417]: https://github.com/rust-lang/rust/issues/33417
[33577]: https://github.com/rust-lang/rust/issues/33577
[34303]: https://github.com/rust-lang/rust/issues/34303
[34511]: https://github.com/rust-lang/rust/issues/34511
[34537]: https://github.com/rust-lang/rust/issues/34537
[34761]: https://github.com/rust-lang/rust/issues/34761
[34981]: https://github.com/rust-lang/rust/issues/34981
[35118]: https://github.com/rust-lang/rust/issues/35118
[35121]: https://github.com/rust-lang/rust/issues/35121
[35203]: https://github.com/rust-lang/rust/issues/35203
[35428]: https://github.com/rust-lang/rust/issues/35428
[35625]: https://github.com/rust-lang/rust/issues/35625
[35896]: https://github.com/rust-lang/rust/issues/35896
[36247]: https://github.com/rust-lang/rust/issues/36247
[36887]: https://github.com/rust-lang/rust/issues/36887
[37390]: https://github.com/rust-lang/rust/issues/37390
[37403]: https://github.com/rust-lang/rust/issues/37403
[37572]: https://github.com/rust-lang/rust/issues/37572
[37854]: https://github.com/rust-lang/rust/issues/37854
[37872]: https://github.com/rust-lang/rust/issues/37872
[38338]: https://github.com/rust-lang/rust/issues/38338
[38356]: https://github.com/rust-lang/rust/issues/38356
[38487]: https://github.com/rust-lang/rust/issues/38487
[38788]: https://github.com/rust-lang/rust/issues/38788
[38864]: https://github.com/rust-lang/rust/issues/38864
[39207]: https://github.com/rust-lang/rust/issues/39207
[39216]: https://github.com/rust-lang/rust/issues/39216
[39412]: https://github.com/rust-lang/rust/issues/39412
[39699]: https://github.com/rust-lang/rust/issues/39699
[40062]: https://github.com/rust-lang/rust/issues/40062
[40180]: https://github.com/rust-lang/rust/issues/40180
[40289]: https://github.com/rust-lang/rust/issues/40289
[40582]: https://github.com/rust-lang/rust/issues/40582
[41022]: https://github.com/rust-lang/rust/issues/41022
[41079]: https://github.com/rust-lang/rust/issues/41079
[41263]: https://github.com/rust-lang/rust/issues/41263
[41517]: https://github.com/rust-lang/rust/issues/41517
[41574]: https://github.com/rust-lang/rust/issues/41574
[41620]: https://github.com/rust-lang/rust/issues/41620
[41758]: https://github.com/rust-lang/rust/issues/41758
[42168]: https://github.com/rust-lang/rust/issues/42168
[42202]: https://github.com/rust-lang/rust/issues/42202
[42238]: https://github.com/rust-lang/rust/issues/42238
[42326]: https://github.com/rust-lang/rust/issues/42326
[42327]: https://github.com/rust-lang/rust/issues/42327
[42524]: https://github.com/rust-lang/rust/issues/42524
[42774]: https://github.com/rust-lang/rust/issues/42774
[42788]: https://github.com/rust-lang/rust/issues/42788
[42877]: https://github.com/rust-lang/rust/issues/42877
[43244]: https://github.com/rust-lang/rust/issues/43244
[43561]: https://github.com/rust-lang/rust/issues/43561
[43606]: https://github.com/rust-lang/rust/issues/43606
[44265]: https://github.com/rust-lang/rust/issues/44265
[44286]: https://github.com/rust-lang/rust/issues/44286
[44582]: https://github.com/rust-lang/rust/issues/44582
[44839]: https://github.com/rust-lang/rust/issues/44839
[44874]: https://github.com/rust-lang/rust/issues/44874
[46043]: https://github.com/rust-lang/rust/issues/46043
[46205]: https://github.com/rust-lang/rust/issues/46205
[46316]: https://github.com/rust-lang/rust/issues/46316
[47115]: https://github.com/rust-lang/rust/issues/47115
[47336]: https://github.com/rust-lang/rust/issues/47336
[47338]: https://github.com/rust-lang/rust/issues/47338
[47402]: https://github.com/rust-lang/rust/issues/47402
[48075]: https://github.com/rust-lang/rust/issues/48075
[48763]: https://github.com/rust-lang/rust/issues/48763
[49359]: https://github.com/rust-lang/rust/issues/49359
[49726]: https://github.com/rust-lang/rust/issues/49726
[50186]: https://github.com/rust-lang/rust/issues/50186
[50202]: https://github.com/rust-lang/rust/issues/50202
[50512]: https://github.com/rust-lang/rust/issues/50512
[50907]: https://github.com/rust-lang/rust/issues/50907
[51085]: https://github.com/rust-lang/rust/issues/51085
[51087]: https://github.com/rust-lang/rust/issues/51087
[51088]: https://github.com/rust-lang/rust/issues/51088
[51443]: https://github.com/rust-lang/rust/issues/51443
[51984]: https://github.com/rust-lang/rust/issues/51984
[52090]: https://github.com/rust-lang/rust/issues/52090
[range_questions]: https://github.com/rust-lang/rust/issues/32311#issuecomment-312388435
[#35237]: https://github.com/rust-lang/rust/issues/35237
[#47043]: https://github.com/rust-lang/rust/issues/47043
[#50882]: https://github.com/rust-lang/rust/issues/50882


