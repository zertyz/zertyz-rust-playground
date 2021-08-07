Documentation file that will be added as a module doc.

Don't forget to build the docs with:

````
cargo doc --document-private-items --features=dox
````

Be aware that Cargo.toml have these lines to produce documentation for tests as well:

````
[features]
dox = []
````