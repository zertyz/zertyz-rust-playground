#![doc = include_str!("../README.md")]
#![recursion_limit="256"]

use std::marker::PhantomPinned;
use std::pin::Pin;
use std::thread::Thread;

fn main() {
    eprintln!("##############################");
    eprintln!("## SELF REFERENTIAL STRUCTS ##");
    eprintln!("##############################");
    eprintln!("The inspiring problem for this play was trying to build such structures that could be safely sent across threads.");
    eprintln!("The official example https://doc.rust-lang.org/std/pin/index.html#example-self-referential-struct instructs us to use 'NonNull<>',");
    eprintln!("But that structure is, purposely, denied from being sent across threads. From its sources:");
    eprintln!("  `NonNull` pointers are not `Sync` because the data they reference may be aliased.");
    eprintln!("Later, using a '*const Type pointer was tried, but that, strangely, also lead to the same error.");
    eprintln!("This play will, then, use as much unsafe code as needed to accomplish this task without using a mutex... lets try, first, with a regular reference:");
    eprintln!("");
    eprintln!("To recap, self-referential structs must assure their value won't be moved after leaving the constructor, where the reference is set. For that, it is");
    eprintln!("allocated on the heap right away (to skip the stack) and enclosed into a Pin. The struct also has a PhantonPinned member, so UnPin is not implemented");
    eprintln!("");

    regular_self_reference();
}

fn regular_self_reference() {

    #[derive(Debug)]
    struct SelfReferencing<'a> {
        data: String,
        data_ref: &'a String,
        _pin: PhantomPinned,
    }

    fn new() -> Pin<Box<SelfReferencing<'static>>> {
        let mut instance = Box::pin(SelfReferencing {
            data: String::from("I'm the real deal"),
            data_ref: unsafe {&*std::ptr::null() as &String},
            _pin: PhantomPinned,
        });
        unsafe {
            let data_ptr = &instance.data as *const String;
            let data_ref = unsafe {&*(data_ptr)};
            let mut_ref: Pin<&mut SelfReferencing> = Pin::as_mut(&mut instance);
            Pin::get_unchecked_mut(mut_ref).data_ref = data_ref;
        }
        instance
    }

    let unmovable = new();
    eprintln!("my unmovable self-referential struct is {:?}", unmovable);

    // across threads test
    std::thread::spawn(move || {
        eprintln!("From a thread, my unmovable self-referential struct is (still) {:?}", unmovable);
    });
    std::thread::sleep(std::time::Duration::from_millis(100));
}