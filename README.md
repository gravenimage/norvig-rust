# norvig-rust

A very naive transliteration of Norvig's spell corrector
(http://norvig.com/spell-correct) into Rust.  And My First Rust
Program :-)

Usage:

```
cargo build --release
./target/release/norvig 1000bad.d
```

There are many, many todos here

 - There's far too much unnecessary reallocation.  (That's one thing I
   definitely like about Rust - allocation patterns are much clearer
   than in the C++ I'm used to).

 - The program is a transliteration of the original Python.  As such it
   feels horribly unidiomatic, but I don't know enough Rust to know
   the proper idioms.

 - I tried to genericize more (at least the `fn known` to take
   `Iterator` instead of `HashSet`) but got in a terrible muddle with
   the borrow-checker.  A lot more to learn I think.

 - HashSets are not deterministic. `HashSet` is parameterized over
   `RandomState` which is injected into the hashing algorithm.  Hence
   two runs can produce two different results - the 'correct' word is
   just the first of the candidates, and if two words have the same
   likelihood either can be chosen. `RandomState` is by default
   initialized with system-derived randomness - I want to explore a
   version of HashSet with the `Default` impl instead.

 - Performance!  It is _slower_ than Python (2.7) by about 20%.  On
   reflection, this isn't as surprising as it could be since Python's
   dict and set code is _highly_ optimized; Norvig's algorithm relies
   heavily on this built-in functionality. And Rust's sets are
   unlikely to be as optimized.  However, this is all guesswork, and
   profiling is in order.  Also, running the Python version uner PyPy
   might be instructive.

Built with rustc 1.0.0-nightly (eaf4c5c78 2015-02-02 15:04:54 +0000)
