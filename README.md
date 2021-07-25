# solana-compute-testbed

test environment for measuring compute units of various actions and tips based on such findins.

# Results

## Working With Vectors

* Initialize your type with the `vec![]` macro, it's the cheapest option available
* Second cheaper option is initializing with a capacity `Vec::with_capacity(...)`, and then pushing
* Most expensive option is initializing with no capacity and then pushing

## Working With For Loops

* As of solana sdk 1.7.4 + rustc 1.53.0 it is cheaper to use for loops with iterators then to access the elements of arrays/vectors directly

## Passing Public Keys To Inner Functions

If you have a function that requires taking a public key as an argument, it is cheaper by around 8 compute units or so to pass it as a reference. The one exception to this however appears providing the public key to a `msg!` statement, for which a public key passed by reference is more expensive.

## Comparing Against `Pubkey::default()`

If you need to compare against `Pubkey::default()` more than once within the same function body, it is cheaper to store the results of `Pubkey::default()` into a variable, and compare against that variable then it is to compare against `Pubkey::default()`

Cheaper:

```rust
let a = Pubkey::default();
assert!(a != some_key);
assert!(a != some_key);
```

Expensive:

```rust
assert!(a != Pubkey::default());
assert!(a != Pubkey::default());
```