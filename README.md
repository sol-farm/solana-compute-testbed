# solana-compute-testbed

test environment for measuring compute units of various actions and tips

# Working With Vector

* Initialize your type with the `vec![]` macro, it's the cheapest option available
* Second cheaper option is initializing with a capacity `Vec::with_capacity(...)`, and then pushing
* Most expensive option is initializing with no capacity and then pushing

# Working With For Loops

* As of solana sdk 1.7.4 + rustc 1.53.0 it is cheaper to use for loops with iterators then to access the elements of arrays/vectors directly