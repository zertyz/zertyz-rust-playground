use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::collections::HashMap;

fn main() {
    //captured_variable_cannot_escape_FnMut_closure_body();
    //borrow_is_only_returned_after_last_lambda_call();
    //immutable_closure_mutating_a_counter_from_two_threads();
    //big_o_simulation();
    //closure_storage_and_passing_back_and_forth();
    generic_callbacks();
}

fn generic_callbacks() {
    println!("With this test I want to demonstrate that a struct is able to receive a callback as a generic, which would enable the maximum optimizations possible");
    println!("-- this is desirable for structs receiving callbacks\n");

    struct Queue<IS_EMPTY>
           where IS_EMPTY: FnMut() -> bool {
        is_empty_callback: IS_EMPTY,
    }

    impl<IS_EMPTY: FnMut() -> bool> Queue<IS_EMPTY> {
        fn new(is_empty_callback: IS_EMPTY) -> Self {
            Queue {
                is_empty_callback
            }
        }
        fn dequeue(&self) {
            let mutable_self = unsafe {&mut *((self as *const Self) as *mut Self)};
            (mutable_self.is_empty_callback)();
        }
    }

    let mut x=10;
    let queue = Queue::new(|| {
        println!("YEAHP! I was called!!!");
        x += 1;
        true
    });

    queue.dequeue();

    println!("x is now {}", x);

    println!("\nNow go and past this code on the compiler explorer https://rust.godbolt.org/, change the callback -- maybe to nothing -- and watch if there are optimizations");
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

/// the motivation for this spike started when working on the ogre-events framework.
/// There, I need to provide closures that are stored on Struct A (the event pipeline);
/// this, then, is moved to Struct B (the event channel) and, finally, a vector of
/// thread loop closures are returned, to call each of the original closures whenever B.produce(payload)
/// arrives.
/// All was working fine with the static version -- using arrays.
/// When I moved to a dynamic version (using vectors), life time problems started popping up.
fn closure_storage_and_passing_back_and_forth() {

    type EventProcessorClosureType<'scope> = dyn FnMut() + 'scope;
    type ThreadLoopClosureType<'scope>     = dyn FnMut() + 'scope;

    /// this one is analogous to the 'build_loop_callback' family of functions
    fn event_processor_to_thread_loop_closure<'a>(mut event_processor: &'a mut (impl FnMut() + 'a)) -> Vec<Box<ThreadLoopClosureType<'a>>> {
        let mut v: Vec<Box<ThreadLoopClosureType<'a>>> = Vec::new();
        v.push(Box::new(move || event_processor()));
        v
    }

    struct A_Array<'a, const LEN: usize> {
        closures: [&'a mut EventProcessorClosureType<'a>;LEN],
    }
    struct B_Array<'a, const LEN: usize> {
        A: A_Array<'a, LEN>,
    }
    fn array_backed_storage() {
        let mut array_callback_ran = false;
        let a = A_Array {
            closures: [
                &mut || {
                    eprintln!("A_Array closure was called");
                    array_callback_ran = true;
                },
            ]
        };
        let mut b = B_Array {
            A: a,
        };
        let mut loops = event_processor_to_thread_loop_closure(&mut b.A.closures[0]);
        for l in loops.iter_mut() {
            l();
        }
        drop(loops);
        drop(b);
        eprintln!("¿array_callback_ran? {}", array_callback_ran);
    }

    struct A_Vec<'a> {
        closures: Vec<Box<EventProcessorClosureType<'a>>>,
    }
    struct B_Vec<'a> {
        A: A_Vec<'a>,
    }
    fn vec_backed_storage() {
        let mut vec_callback_ran = false;
        let a = A_Vec {
            closures: vec![
                Box::new(|| {
                    eprintln!("A_Vec closure was called");
                    vec_callback_ran = true;
                }),
            ]
        };
        let mut b = B_Vec {
            A: a,
        };
        let mut loops = event_processor_to_thread_loop_closure(&mut b.A.closures[0]);
        for l in loops.iter_mut() {
            l();
        }
        drop(loops);
        drop(b);
        eprintln!("¿vec_callback_ran? {}", vec_callback_ran);
    }

    array_backed_storage();
    vec_backed_storage();
}
