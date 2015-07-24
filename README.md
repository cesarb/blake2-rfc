# blake2-rfc

This is a pure Rust implementation of BLAKE2 based on the [draft RFC]
for the BLAKE2 hash functions.

[draft RFC]: https://tools.ietf.org/html/draft-saarinen-blake2

## Design

This crate follow the common API design for streaming hash functions,
which has one state/context struct and three associated functions: one
to initialize the struct, one which is called repeatedly to process the
incoming data, and one to do the final processing and return the hash.
For the case where the full data is already in memory, there is a
convenience function which does these three steps in a single call.

This basic design was slightly adapted to make a better use of Rust's
characteristics: the finalization function consumes the struct, doing a
move instead of a borrow, so the struct cannot be accidentally used
after its internal state has been overwritten by the finalization.

To prevent timing attacks, it's important that the comparison of hash
values takes constant time. To make it easier to do the right thing, the
finalization function returns the result wrapped in a struct which does
a constant-time comparison by default. If a constant-time comparison is
not necessary, the hash result can easily be extracted from this struct.

## Limitations

A single BLAKE2b hash is limited to 16 exabytes, lower than its
theorical limit (but identical to the BLAKE2s theorical limit), due to
the use of a `u64` as the byte counter. This limit can be increased, if
necessary, after either the `extprim` crate (with its `u128` type) or
the `OverflowingOps` trait become usable with the "stable" Rust release.

This crate does not attempt to clear potentially sensitive data from its
work memory (which includes the state context, the stack, and processor
registers). To do so correctly without a heavy performance penalty would
require help from the compiler. It's better to not attempt to do so than
to present a false assurance.

## Non-RFC uses

This crate is limited to the features described in the draft RFC: only
the "digest length" and "key length" parameters can be used.

If you need to use other advanced BLAKE2 features, this crate has an
undocumented function to create a hashing context with an arbitrary
parameter block, and an undocumented function to finalize the last node
in tree hashing mode. You are responsible for creating a valid parameter
block, for hashing the padded key block if using keyed hashing, and for
calling the correct finalization function. The parameter block is not
validated by these functions.
