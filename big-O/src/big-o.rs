pub mod big_o_analysis;
mod conditionals;

use crate::big_o_analysis::{BigOAlgorithmComplexity, SetResizingAlgorithmMeasurements, ConstantSetAlgorithmMeasurements, BigOAlgorithmAnalysis};
use crate::conditionals::OUTPUT;

use std::convert::TryInto;
use std::ops::Range;
use std::time::SystemTime;
use std::io;
use std::io::Write;
use std::iter::Rev;

pub type AlgorithmFnPtr = fn(u32) -> u32;

pub fn analyze_crud_algorithm<'a,
                              _ResetClosure:  FnMut(u32) -> u32,
                              _CreateClosure: FnMut(u32) -> u32,
                              _ReadClusure:   FnMut(u32) -> u32,
                              _UpdateClosure: FnMut(u32) -> u32,
                              _DeleteClosure: FnMut(u32) -> u32,
                              T: TryInto<u64> > (
                                                 crud_name: &'a str,
                                                 reset_fn:  &mut _ResetClosure,
                                                 create_fn: &mut _CreateClosure,
                                                 read_fn:   &mut _ReadClusure,
                                                 update_fn: &mut _UpdateClosure,
                                                 delete_fn: &mut _DeleteClosure,
                                                 warmup_iterations:  u32,
                                                 create_iterations:  u32,
                                                 read_iterations:    u32,
                                                 update_iterations:  u32,
                                                 delete_iterations:  u32,
                                                 time_unit: &TimeUnit<T>)
    -> (BigOAlgorithmAnalysis<SetResizingAlgorithmMeasurements<'a>>,
        BigOAlgorithmAnalysis<ConstantSetAlgorithmMeasurements<'a>>,
        BigOAlgorithmAnalysis<ConstantSetAlgorithmMeasurements<'a>>,
        BigOAlgorithmAnalysis<SetResizingAlgorithmMeasurements<'a>>) {

    /// wrap around the original 'run_pass' to output intermediate results
    fn _run_pass<_AlgorithmClosure, T>(result_prefix: &str, result_suffix: &str, algorithm: &mut _AlgorithmClosure, algorithm_type: &BigOAlgorithmType, range: Range<u32>, unit: &TimeUnit<T>)
        -> (u64, u32) where _AlgorithmClosure: FnMut(u32) -> u32, T: TryInto<u64> {
        let (pass_elapsed_us, r) = run_pass(algorithm, algorithm_type, range, unit);
        OUTPUT(&format!("{}{}{}{}", result_prefix, pass_elapsed_us, unit.unit_str, result_suffix));
        (pass_elapsed_us, r)
    }

    const NUMBER_OF_PASSES: u32 = 2;

    let mut create_elapsed_passes = [0u64; NUMBER_OF_PASSES as usize];
    let mut   read_elapsed_passes = [0u64; NUMBER_OF_PASSES as usize];
    let mut update_elapsed_passes = [0u64; NUMBER_OF_PASSES as usize];
    let mut delete_elapsed_passes = [0u64; NUMBER_OF_PASSES as usize];

    // computed result to avoid any call cancellation optimizations when running in release mode
    let mut r: u32 = 0;

    OUTPUT(&format!("{} CRUD Algorithm Complexity Analysis:\n  ", crud_name));

    for pass in 0..NUMBER_OF_PASSES {
        // warmup only on the first pass
        if pass == 0 {
            OUTPUT("warm");
            io::stdout().flush().unwrap();
            reset_fn(0);
            let (_warmup_elapsed, wr) = _run_pass("up: ", "; ", create_fn, &BigOAlgorithmType::SetResizing, 0..warmup_iterations, time_unit);
            r += wr;
            reset_fn(warmup_iterations);
        }
        // show pass number
        OUTPUT(&format!("{} Pass (", if pass == 0 {
            "First"
        } else {
            "); Second"
        }));
        // execute passes
        let (create_elapsed, cr) = _run_pass("create: ", "", create_fn, &BigOAlgorithmType::SetResizing, create_iterations*pass..create_iterations*(pass+1), time_unit);
        let (read_elapsed,   rr) = _run_pass("; read: ", "", read_fn, &BigOAlgorithmType::SetResizing, read_iterations*pass..read_iterations*(pass+1), time_unit);
        let (update_elapsed, ur) = _run_pass("; update: ", "", update_fn, &BigOAlgorithmType::SetResizing, update_iterations*pass..update_iterations*(pass+1), time_unit);

        create_elapsed_passes[pass as usize] = create_elapsed;
        read_elapsed_passes[pass as usize]   = read_elapsed;
        update_elapsed_passes[pass as usize] = update_elapsed;

        r += cr^rr^ur;
    }
    OUTPUT("):\n\n");

    // analyze & output "create", "read" and "update" reports
    let create_analysis = big_o_analysis::analyse_set_resizing_algorithm(SetResizingAlgorithmMeasurements {
        measurement_name: "Create",
        pass_1_total_time: create_elapsed_passes[0],
        pass_2_total_time: create_elapsed_passes[1],
        delta_set_size: create_iterations
    });
    let read_analysis = big_o_analysis::analyse_constant_set_algorithm(ConstantSetAlgorithmMeasurements {
        measurement_name: "Read",
        pass_1_total_time: read_elapsed_passes[0],
        pass_2_total_time: read_elapsed_passes[1],
        pass_1_set_size: create_iterations,
        pass_2_set_size: create_iterations*2,
        repetitions: read_iterations,
    });
    let update_analysis = big_o_analysis::analyse_constant_set_algorithm(ConstantSetAlgorithmMeasurements {
        measurement_name: "Update",
        pass_1_total_time: update_elapsed_passes[0],
        pass_2_total_time: update_elapsed_passes[1],
        pass_1_set_size: create_iterations,
        pass_2_set_size: create_iterations*2,
        repetitions: update_iterations,
    });
    if create_iterations > 0 {
        OUTPUT(&format!("{}\n\n", create_analysis));
    }
    if read_iterations > 0 {
        OUTPUT(&format!("{}\n\n", read_analysis));
    }
    if update_iterations > 0 {
        OUTPUT(&format!("{}\n\n", update_analysis));
    }

    // delete passes (note that delete passes are applied in reverse order)
    if delete_iterations > 0 {
        OUTPUT("Delete Passes (");
        for pass in (0..NUMBER_OF_PASSES).rev() {
            let msg = format!("{}: ", if pass == 0 {
                "; 1st"
            } else {
                "2nd"
            });
            let (delete_elapsed, dr) = _run_pass(&msg, "", delete_fn, &BigOAlgorithmType::SetResizing, delete_iterations * (pass + 1) .. delete_iterations * pass, time_unit);
            delete_elapsed_passes[pass as usize] = delete_elapsed;
            r += dr;
        }
    }

    OUTPUT(&format!(") r={}:\n", r));

    // analyze & output "delete" report
    let delete_analysis = big_o_analysis::analyse_set_resizing_algorithm(SetResizingAlgorithmMeasurements {
        measurement_name: "Delete",
        pass_1_total_time: delete_elapsed_passes[0],
        pass_2_total_time: delete_elapsed_passes[1],
        delta_set_size: delete_iterations,
    });
    if delete_iterations > 0 {
        OUTPUT(&format!("{}\n\n", delete_analysis));
    }

    (create_analysis, read_analysis, update_analysis, delete_analysis)
}

/// Runs a pass on the given 'algorithm' callback function (see [AlgorithmFnPtr]),
/// measuring (and returning) the time it took to run all iterations specified in 'range'.
/// ````
///     /// Algorithm function under analysis -- receives the iteration number on each call
///     /// (for set changing algorithms) or the set size (for constant set algorithms).
///     /// Returns any computed number to avoid compiler call cancellation optimizations
///     fn algorithm(i: u32) -> u32 {0}
/// ````
/// returns: tuple with (elapsed_time: u64, computed_number: u32)
fn run_pass<T, _AlgorithmClosure>
           (algorithm: &mut _AlgorithmClosure, algorithm_type: &BigOAlgorithmType, range: Range<u32>, unit: &TimeUnit<T>)
            -> (u64, u32) where T: TryInto<u64>, _AlgorithmClosure: FnMut(u32) -> u32 {
    let mut r: u32 = range.end;
    let pass_start = SystemTime::now();

    // run 'algorithm()' allowing normal or reversed order
    match algorithm_type {
        BigOAlgorithmType::ConstantSet => {
            if range.end < range.start {
                for e in (range.end..range.start).rev() {
                    r ^= algorithm(e);
                }
            } else {
                for e in range {
                    r ^= algorithm(e);
                }
            }
        },
        BigOAlgorithmType::SetResizing => {
            if range.end < range.start {
                for e in (range.end..range.start).rev() {
                    r ^= algorithm(e);
                }
            } else {
                for e in range {
                    r ^= algorithm(e);
                }
            }
        },
    }

    let pass_end = SystemTime::now();
    let duration = pass_end.duration_since(pass_start).unwrap();
    let pass_elapsed = (unit.duration_conversion_fn_ptr)(&duration).try_into().unwrap_or_default();
    (pass_elapsed, r)
}

/// Specifies if the algorithm under analysis alters the data set it works on or if it has no side-effects on it
/// Different math applies on each case, as well as different parameters to the 'algorithm(u32) -> u32' function.
pub enum BigOAlgorithmType {
    /// the algorithm under analysis change the data set size it operates on. Examples: insert/delete, enqueue/dequeue, ...
    SetResizing,
    /// the algorithm under analysis doesn't change the data set size it operates on. Examples: queries, sort, fib, ...
    ConstantSet,
}

/// Specifies a time unit for the 'big-O' crate when measuring / reporting results.
/// Please use one of the prebuilt 'TimeUnits' constants:
/// TimeUnits::NANOSECOND, TimeUnits::MICROSECOND,TimeUnits::MILLISECOND,  TimeUnits::SECOND
pub struct TimeUnit<T> {
    /// printable unit
    unit_str: &'static str,
    /// on std::time::Duration 'as_micros', 'as_seconds', ... function to convert a Duration object into a scalar
    duration_conversion_fn_ptr: fn(&std::time::Duration) -> T,
}
/// prebuilt 'TimeUnit' constants
struct TimeUnits {}
impl TimeUnits {
    pub const NANOSECOND:  TimeUnit<u128> = TimeUnit { unit_str: "ns", duration_conversion_fn_ptr: std::time::Duration::as_nanos};
    pub const MICROSECOND: TimeUnit<u128> = TimeUnit { unit_str: "µs", duration_conversion_fn_ptr: std::time::Duration::as_micros};
    pub const MILLISECOND: TimeUnit<u128> = TimeUnit { unit_str: "ms", duration_conversion_fn_ptr: std::time::Duration::as_millis};
    pub const SECOND:      TimeUnit<u64>  = TimeUnit { unit_str: "s",  duration_conversion_fn_ptr: std::time::Duration::as_secs};
}



fn main() {
    println!("Welcome to the big-O notation spikes!");
    println!();
    //tests::lowLevelExperiments();
}

#[cfg(test)]
mod tests {
    use super::*;

    use serial_test::serial;
    use std::sync::Arc;

    #[test]
    #[serial(cpu)]
    fn analyze_crud_algorithm_output_check() {
        let iterations = 100000;
        let mut vec = Arc::new(parking_lot::RwLock::new(Vec::<u32>::with_capacity(iterations)));
        let mut reset_vec = Arc::clone(&vec);
        let mut create_vec = Arc::clone(&vec);
        let read_vec = Arc::clone(&vec);
        let mut update_vec = Arc::clone(&vec);
        let mut delete_vec = Arc::clone(&vec);
        drop(vec);
        analyze_crud_algorithm("Push & Pop (best case) Vec with ParkingLot",
            &mut |n| {
                let mut vec = reset_vec.write();
                vec.clear();
                vec.len() as u32
            },
            &mut |n| {
                let mut vec = create_vec.write();
                vec.push(n);
                vec.len() as u32
            },
            &mut |n| {
                let vec = read_vec.read();
                vec[n as usize]
            },
            &mut |n| {
                let mut vec = update_vec.write();
                vec[n as usize] = n+1;
                vec.len() as u32
            },
            &mut |n| {
                let mut vec = delete_vec.write();
                vec.remove(n as usize);
                vec.len() as u32
            },
            (iterations / 100) as u32,
            iterations as u32,
            iterations as u32,
            iterations as u32,
            iterations as u32,
            &TimeUnits::MICROSECOND);
    }

    #[test]
    #[serial(cpu)]
    fn vec_worse_case() {
        let iterations = 100000;
        let mut vec = Arc::new(parking_lot::RwLock::new(Vec::<u32>::with_capacity(iterations)));
        let mut reset_vec = Arc::clone(&vec);
        let mut create_vec = Arc::clone(&vec);
        let read_vec = Arc::clone(&vec);
        let mut update_vec = Arc::clone(&vec);
        let mut delete_vec = Arc::clone(&vec);
        drop(vec);
        analyze_crud_algorithm("Insert & Remove (worse case) Vec with ParkingLot",
                               &mut |n| {
                                   let mut vec = reset_vec.write();
                                   vec.clear();
                                   vec.len() as u32
                               },
                               &mut |n| {
                                   let mut vec = create_vec.write();
                                   vec.insert(0, (iterations as u32)*2 - n);
                                   vec.len() as u32
                               },
                               &mut |n| {
                                   let vec = read_vec.read();
                                   vec[n as usize]
                               },
                               &mut |n| {
                                   let mut vec = update_vec.write();
                                   vec[n as usize] = n+1;
                                   vec.len() as u32
                               },
                               &mut |n| {
                                   let mut vec = delete_vec.write();
                                   vec.remove(0);
                                   vec.len() as u32
                               },
                               (iterations / 100) as u32,
                               iterations as u32,
                               iterations as u32,
                               iterations as u32,
                               iterations as u32,
                               &TimeUnits::MICROSECOND);
    }

}