# internal

Private fields were a mistake.

## What?

Ok, maybe that's a bit of an exaggeration.

Private fields exist for two reasons:

1. To allow library developers to make breaking changes to things without 
   making major [semver](https://semver.org/) jumps.
2. To enable compiler optimizations.

But there's a better way.

The main problem with private fields is, well, it makes things private. 
It's too easy to lock potentially useful functionality away from your users.

A solution to that problem is internal fields, but Rust doesn't have that 
feature. This crate brings it to Rust via a proc macro and a feature flag.

### How do internal fields work?

By default, internal fields can't be accessed, but they can be enabled and 
used when absolutely necessary.

It's a balanced solution to both ease of library development and freedom 
of library usage. It's easy to tell what could change, but nobody's 
limited or burdened. On top of all this, the compiler can still take 
advantage of performance improvements when internal fields aren't accessed.

The `internal` crate does it "the Rust way" by exposing the fields only 
when the `"internal"` feature for the library is enabled. It also adds a 
doc comment warning to the top of all internal fields to make it clear 
when something is internal.

## Usage

To mark something as internal, use the `internal` proc macro. It 
effectively replaces private fields because those are useless when 
internal fields exist.

The macro works recursively to mark everything under it that's private as 
internal instead. So if you define `#[internal] mod stuff {...}`, anything 
inside and including `stuff` that's private will become internal. If you 
were to make `stuff` public, it would always be public itself, but still 
apply internal recursively.

### `your_lib`

```rs
use internal::internal as int;

#[int]
fn internal_fn(arg: InternalStruct) {
	// ...
}

#[int]
#[derive(Clone)]
struct InternalStruct {
	field: PrivateThing
}

#[int]
mod private_mod {
   pub struct PublicStruct {
      private_field: PublicThing
   }
}
```

### `consumer`

```toml
# Cargo.toml

your_lib = { features = ["internal"] }
```
```rs
// mod.rs

// If the `internal` feature is explicitly enabled,
// anything marked as internal will become public.
use your_lib::{internal, InternalStruct, PublicStruct};

internal_fn(InternalStruct {
	field: ...
});

// Everything gets publicized recursively.
private_mod::PublicStruct {
	private_field: ...
}
```
