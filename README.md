Here is a set of small programs that exercise various aspects of the Rust programming language, which I used while learning it...

... or even nowadays, to dismiss any undocumented behavior or to delve into an interesting crate. 

I'm sharing them for it might be useful to someone, someday.

Contents:
--------

    Rust 2018:
        Complex Project Setup -- see the root Cargo.toml
        meta-programming -- trying to match the same power we have in C++
        lifetimes -- lerning some tricks in order (not to fight) to work along with the borrow checker
        mmap-containers -- experiments with mmaped containers
        databases -- trying some NoSQL (object) embedded database crates...
        mmapped-cached-crate -- exploring the wonderful 'cached' crate and trying it with a mmap backed cache store
        big-O -- big-O crate spikes
        first-class-closures -- exploring closures as first-class values, including the possibility & circumsstances of their execution in different threads
        cpp-interop -- aiming at the higher efficiency possible

    Rust 2021:
        web-backend-rocket -- a JSON backend using Rocket for HTML 5 applications
        rust-mt5-bridge -- an example of how to generate DLLs for use in MetaTrader 5


# Amusements

## to generate the program module graphs:
## =====================================
```
package="big-O"; for layout in dot neato twopi circo fdp sfdp; do for format in svg pdf; do cargo modules generate graph --all-features --with-uses --with-types --with-tests --layout ${layout} --package "$package" | grep -v -E "\"${package}\" |label=\"owns\"" | sed 's|label="uses"|label=""|g' | sed 's|splines="line"|splines=true|g' | sed "s|${package}::||g" | sed 's|::|⸬|g' | dot -x -T${format} -o ${layout}.${format}; done; done
```

### install the 'cargo-modules' tool with:
```
cargo install cargo-modules
```

## to generate the program dependencies graph:
## ==========================================
```
cargo depgraph --all-features --all-deps | dot -x -Tpdf -o deps.pdf
```

### install 'cargo-depgraph' from aur or like we did for 'cargo-modueles' above
