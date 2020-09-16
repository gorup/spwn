# `spwn`

An attempt to abstract away the spawning of asynchronous tasks in Rust.

# Using `spwn`
There is one setup thing to do, this needs to be done before anything is attempted to be `spwn`ed.


```
spwn::set_spwner(Spwner::Tokio).unwrap();

// or

spwn::set_spwner(Spwner::AsyncStd).unwrap();
```

Besides that setup, you just `spwn` things, and await their result if you'd like to, or not.

# Testing / Seeing what libs run on what runtimes
```
cargo test --no-fail-fast
```

# What Runs On What

| | tokio | async_std |
--- | --- | ---
| tokio tcp | yes | no |
| hyper | yes | no |
| asyncstd tcp | yes | yes |
| surf & tide | yes | yes |
