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