use cached::proc_macro::cached;
use cached::SizedCache;

use std::thread;
use std::time::Duration;

fn main() {
    eprintln!("############################");
    eprintln!("## MMAPPED 'cached' CRATE ##");
    eprintln!("############################");
    eprintln!();

    test_standard_cache_settings();
}

fn test_standard_cache_settings() {

    println!("## Standard Cache Settings:");
    println!("###########################");
    println!("# on this test I attempt to have a copy & paste model I can put everywhere");
    println!("# and customize from there on without having to refer to the documentation.");
    println!("# BTW, how cool this language is for allowing such a plug'n'play that easy?");

    #[cached(/*size=1024, time=3660, result=true, option=true*/)]
    fn standard_keys_pure_function(n: u32) -> u32 {
        thread::sleep(Duration::from_secs(1));
        n+1
    }

    eprintln!("standard_keys_pure_function(0)={}", standard_keys_pure_function(0));
    eprintln!("standard_keys_pure_function(1)={}", standard_keys_pure_function(1));
    eprintln!("standard_keys_pure_function(2)={}", standard_keys_pure_function(2));
    eprintln!("standard_keys_pure_function(3)={}", standard_keys_pure_function(3));
    eprintln!("standard_keys_pure_function(4)={}", standard_keys_pure_function(4));
    eprintln!("standard_keys_pure_function(5)={}", standard_keys_pure_function(5));
    eprintln!("--> Again, this time without sleeping a second between each -- using the cached value:");
    eprintln!("standard_keys_pure_function(0)={}", standard_keys_pure_function(0));
    eprintln!("standard_keys_pure_function(1)={}", standard_keys_pure_function(1));
    eprintln!("standard_keys_pure_function(2)={}", standard_keys_pure_function(2));
    eprintln!("standard_keys_pure_function(3)={}", standard_keys_pure_function(3));
    eprintln!("standard_keys_pure_function(4)={}", standard_keys_pure_function(4));
    eprintln!("standard_keys_pure_function(5)={}", standard_keys_pure_function(5));

    #[cached(
        type = "SizedCache<String, u32>",
        create = "{ SizedCache::with_size(100) }",
        convert = r#"{ format!("{}", n/2) }"#
        /*size=1024, time=3660, result=true, option=true*/)]
    fn custom_keys_pure_function(n: u32, counter: &mut u32) -> u32 {
        thread::sleep(Duration::from_secs(1));
        *counter = *counter + 1;
        *counter + n - 1
    }

    let mut counter = 0;

    let mut show = |prefix: &str, n: u32| {
        let counter_copy = counter;
        eprintln!("{}custom_keys_pure_function({}, {})={}", prefix, n, counter_copy, custom_keys_pure_function(n, &mut counter));
    };

    show("uncached ", 0);
    show("uncached ", 2);
    show("uncached ", 4);
    show("cached   ", 1);
    show("cached   ", 3);
    show("cached   ", 5);
    eprintln!("--> Again, this time all cached (note the speed as well as the 'counter'):");
    show("", 0);
    show("", 2);
    show("", 4);
    show("", 1);
    show("", 3);
    show("", 5);
}
