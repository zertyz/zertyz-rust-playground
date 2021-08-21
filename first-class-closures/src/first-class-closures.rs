use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::collections::HashMap;

fn main() {
    //captured_variable_cannot_escape_FnMut_closure_body();
    //borrow_is_only_returned_after_last_lambda_call();
    //immutable_closure_mutating_a_counter_from_two_threads();
    big_o_simulation();
}

struct ClosureSimulator<P, R> {
    fn_ptr: fn(&Self) -> R,
    data: P,
}

struct MutClosureSimulator<P, R> {
    fn_ptr: fn(&mut Self) -> R,
    data: P,
}

fn big_o_simulation() {

    let map_locker = Arc::new(parking_lot::RwLock::new(HashMap::<String, u32>::new()));

    let closure = |n| {
        //let arc = Arc::clone(&map_locker);
        let mut hashmap = map_locker.write();
        hashmap.insert(format!("key for {}", n), n);
        hashmap.len() as u32
    };

    for i in 0..2 {
        closure_caller(i, &closure);
    }

    println!("at the end, my map is: {:?}", map_locker.read());
}

fn closure_caller<T: Fn(u32) -> u32 + Sync>(n:u32, closure: &T) {
    crossbeam::scope(|scope| {
        let handler1 = scope.spawn(|_| closure(n+41));
        let handler2 = scope.spawn(|_| closure(n+42));
        handler1.join();
        handler2.join();
    }).unwrap();

}

/// to activate the error message bellow, uncomment the line 'x += 1;'
/// ```text
/// error: captured variable cannot escape `FnMut` closure body
///   --> first-class-closures/src/first-class-closures.rs:18:9
///    |
/// 13 |     let mut x = 10;
///    |         ----- variable defined here
/// 14 |     //let immutableClosure = || println!("immutableClosure says: x={}", x);
/// 15 |     let mut mutableSimulationClosure = || {
///    |                                         - inferred to be a `FnMut` closure
/// 16 |         x += 1;
///    |         - variable captured here
/// 17 |         println!("mutableSimulationClosure says: x is now {}", x);
/// 18 |         || println!("immutableClosure says: x={}", x)
///    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ returns a closure that contains a reference to a captured variable, which then escapes the closure body
///    |
///    = note: `FnMut` closures only have access to their captured variables while they are executing...
///    = note: ...therefore, they cannot allow references to captured variables to escape
fn captured_variable_cannot_escape_FnMut_closure_body() {
    let mut x = 10;
    //let immutableClosure = || println!("immutableClosure says: x={}", x);
    let mut mutableSimulationClosure = || {
        //x += 1;
        println!("mutableSimulationClosure says: x is now {}", x);
        || println!("immutableClosure says: x={}", x)
    };

    println!("Hello, world:");
    let immutableClosure = mutableSimulationClosure();
    immutableClosure();
    let immutableClosure = mutableSimulationClosure();
    immutableClosure();
    println!("Main attests that x is {}", x);
    println!("Conclusion: closures that only reads a variable are freely allowed to share its reference or ownership (controlled automatically by rust)");
    println!("Conclusion: but for mutable variables, this case, if accepted, would cause no harm: the returned closure would be the owner of the variable...");
}

/// to activate the error message bellow, uncomment the println! line
/// ``` text
/// error[E0502]: cannot borrow `x` as immutable because it is also borrowed as mutable
///   --> first-class-closures/src/first-class-closures.rs:64:43
///    |
/// 57 |     let mut mutableSimulationClosure = || {
///    |                                        -- mutable borrow occurs here
/// 58 |         x += 1;
///    |         - first borrow occurs due to use of `x` in closure
/// ...
/// 64 |     println!("Main attests that x is {}", x);
///    |                                           ^ immutable borrow occurs here
/// 65 |     mutableSimulationClosure();
///    |     ------------------------ mutable borrow later used here
fn borrow_is_only_returned_after_last_lambda_call() {
    let mut x = 10;
    //let immutableClosure = || println!("immutableClosure says: x={}", x);
    let mut mutableSimulationClosure = || {
        x += 1;
        println!("mutableSimulationClosure says: x is now {}", x);
    };

    println!("Hello, world:");
    mutableSimulationClosure();
    //println!("Main attests that x is {}", x);
    mutableSimulationClosure();
    println!("Main attests that x is {}", x);
    println!("Conclusion: this function's title says it all...");
}

fn immutable_closure_mutating_a_counter_from_two_threads() {
    let x = Arc::new(AtomicU32::new(10));
    let immutableClosure = || {
        let x = Arc::clone(&x);
        x.fetch_add(1, Ordering::Relaxed);
        println!("immutableClosure says: new x is {}", x.fetch_add(0, Ordering::Relaxed));
    };

    crossbeam::scope(|scope| {
        let handler1 = scope.spawn(|_| immutableClosure());
        let handler2 = scope.spawn(|_| immutableClosure());
        handler1.join();
        handler2.join();
    }).unwrap();

    println!("Main attests that x is {}", x.fetch_add(0, Ordering::Relaxed));

    println!("Conclusion: here was demonstrated the a closure may be called by two threads and may operate on an Arc enclosed value");
    println!("            while still preserving it's 'immutable' properties");
}
