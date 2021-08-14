//! Contains the "low level" artifacts for analyzing an algorithm's time measurements, in Big-O notation:\
//! [analyse_constant_set_algorithm] & [analyse_set_resizing_algorithm] -- functions performing the analysis;\
//! [ConstantSetAlgorithmMeasurements] & [SetResizingAlgorithmMeasurements] --structs for holding the measurements;\
//! [BigOAlgorithmComplexity] -- analysis result enum & pretty str methods.\

/// acceptable variance (or errors) when measuring times
const PERCENT_TOLERANCE: f64 = 0.10;

#[derive(Debug, PartialEq)]
pub enum BigOAlgorithmComplexity {
    BetterThanO1,
    O1,
    OLogN,
    BetweenOLogNAndON,
    ON,
    WorseThanON
}

impl BigOAlgorithmComplexity {

    /// verbose description of each enum element
    pub fn as_pretty_str(&self) -> &'static str {
        match self {
            BigOAlgorithmComplexity::BetterThanO1      => "Better than O(1) -- aren't the machines idle? too many threads? too little RAM?",
            BigOAlgorithmComplexity::O1                => "O(1)",
            BigOAlgorithmComplexity::OLogN             => "O(log(n))",
            BigOAlgorithmComplexity::BetweenOLogNAndON => "Worse than O(log(n)) but better than O(n)",
            BigOAlgorithmComplexity::ON                => "O(n)",
            BigOAlgorithmComplexity::WorseThanON       => "Worse than O(n)",
        }
    }
}

/// Represents the measurements made on Algorithms that don't alter the set size of the data they operate on
/// (Selects / Updates / Sort / Fib...). This struct keeps average times (instead of best times) and it should
/// be used on latency variation conditions (network, several threads, ...) and also for searches (naive
/// implementations might find the first element immediately)
pub struct ConstantSetAlgorithmMeasurements<'a> {
    /// a name for this measurement, for presentation purposes
    pub measurement_name:   &'a str,
    /// the absolute time (same unit as 'pass_2_total_time') it took to run "pass 1" (an operation applied
    /// 'repetitions' times, on an universe of 'pass_1_set_size' elements)
    pub pass_1_total_time:      u64,
    /// the absolute time (same unit as 'pass_1_total_time') it took to run "pass 2" (an operation applied
    /// 'repetitions' times, on an universe of 'pass_2_set_size' elements)
    pub pass_2_total_time:      u64,
    /// set size when running "pass 1" (an operation repeated 'repetitions' times)
    pub pass_1_set_size:    u32,
    /// set size when running "pass 2" (an operation repeated 'repetitions' times)
    pub pass_2_set_size:    u32,
    /// number of times the algorithm ran on each pass;
    /// each algorithm iteration should behave as executing on the same element without leaving side-effects
    pub repetitions: u32,
}

/// Performs the algorithm analysis for a reasonably large select/update operation (on a database or not).
/// To perform the analysis, two passes of selects/updates of r elements must be done.
/// On the first pass, the data set must have 'n1' elements and, on the second pass, 'n2' elements -- 'n2' must be (at least) twice 'n1'.
/// 'r' should be reasonably large so that end-start can be accurately measured and account for OS, IO and network latencies.
/// 'start's 1 & 2 and 'end's 1 & 2 are measurement times, regardless of the measurement unit -- milliseconds or microseconds.
/// The returned algorithm complexity is an indication of the time taken to select/update one element on a data set containing
/// 'n' elements, where 'O' is the constant of proportionality -- the average time to select/update 1 element.\
/// Returns: [1] -- the algorithm complexity;\
///          [2] -- a string with the algorithm analysis report.\
pub fn analyse_constant_set_algorithm<'a>(measurements: &'a ConstantSetAlgorithmMeasurements<'a>) -> (BigOAlgorithmComplexity, String) {

    // time variation
    let t1 = measurements.pass_1_total_time as f64 / measurements.repetitions as f64;
    let t2 = measurements.pass_2_total_time as f64 / measurements.repetitions as f64;

    // set size variation
    let n1 = std::cmp::min(measurements.pass_1_set_size, measurements.pass_2_set_size) as f64;
    let n2 = std::cmp::max(measurements.pass_1_set_size, measurements.pass_2_set_size) as f64;

    let mut computed_complexity: BigOAlgorithmComplexity;

    if ((t1/t2) - 1.0_f64) >= PERCENT_TOLERANCE {
        // sanity check
        computed_complexity = BigOAlgorithmComplexity::BetterThanO1;
    } else if ((t2/t1) - 1.0_f64).abs() < PERCENT_TOLERANCE {
        // check for O(1) -- t2/t1 ~= 1
        computed_complexity = BigOAlgorithmComplexity::O1;
    } else if ( ((t2/t1) / ( n2.log2() / n1.log2() )) - 1.0_f64 ).abs() < PERCENT_TOLERANCE {
        // check for O(log(n)) -- (t2/t1) / (log(n2)/log(n1)) ~= 1
        computed_complexity = BigOAlgorithmComplexity::OLogN;
    } else if ( ((t2/t1) / (n2 / n1)) - 1.0_f64 ).abs() < PERCENT_TOLERANCE {
        // check for O(n) -- (t2/t1) / (n2/n1) ~= 1
        computed_complexity = BigOAlgorithmComplexity::ON;
    } else if ( ((t2/t1) / (n2 / n1)) - 1.0_f64 ) > PERCENT_TOLERANCE {
        // check for worse than O(n)
        computed_complexity = BigOAlgorithmComplexity::WorseThanON;
    } else {
        // by exclusion...
        computed_complexity = BigOAlgorithmComplexity::BetweenOLogNAndON;
    }

    let report = format!("'{}' constant set algorithm measurements:\n\
                                 pass         Δt            Σn            ⊆r            t⁻\n\
                                 1) {:>12}  {:>12}  {:>12}  {:>12.3}\n\
                                 2) {:>12}  {:>12}  {:>12}  {:>12.3}\n\
                                 --> Algorithm Analysis: {}\n",
                                measurements.measurement_name,
                                measurements.pass_1_total_time, measurements.pass_1_set_size, measurements.repetitions, t1,
                                measurements.pass_2_total_time, measurements.pass_2_set_size, measurements.repetitions, t2,
                                BigOAlgorithmComplexity::as_pretty_str(&computed_complexity));

    (computed_complexity, report)
}

/// Represents the measurements made on Algorithms that alters the set size of the data they operate on
/// (Inserts / Deletes / Pushes / Pops / Enqueues / Dequeues...)
pub struct SetResizingAlgorithmMeasurements<'a> {
    /// a name for this measurement, for presentation purposes
    pub measurement_name:   &'a str,
    /// the absolute time (same unit as 'pass_2_total_time') it took to run "pass 1" (an operation repeated
    /// 'processing_subset_size' times, leaving 'pass_1_end_set_size' elements at the end)
    pub pass_1_total_time:      u64,
    /// the absolute time (same unit as 'pass_1_total_time') it took to run "pass 2" (an operation repeated
    /// 'processing_subset_size' times, leaving 'pass_2_end_set_size' elements at the end)
    pub pass_2_total_time:      u64,
    /// number of elements added / removed on each pass;
    /// each algorithm iteration should either add or remove a single element
    /// and the test set must start or end with 0 elements
    pub delta_set_size: u32,
}

pub fn analyse_set_resizing_algorithm<'a>(measurements: &'a SetResizingAlgorithmMeasurements<'a>) -> (BigOAlgorithmComplexity, String) {

    let n = measurements.delta_set_size as f64;

    // time variation
    let t1 = measurements.pass_1_total_time as f64 / n;
    let t2 = measurements.pass_2_total_time as f64 / n;


    let mut computed_complexity: BigOAlgorithmComplexity;

    if ((t1/t2) - 1.0_f64) >= PERCENT_TOLERANCE {
        // sanity check
        computed_complexity = BigOAlgorithmComplexity::BetterThanO1;
    } else if ((t2/t1) - 1.0_f64).abs() < PERCENT_TOLERANCE {
        // check for O(1) -- t2/t1 ~= 1
        computed_complexity = BigOAlgorithmComplexity::O1;
    } else if ( ((t2/t1) / ( (n * 3.0_f64).log2() / n.log2() )) - 1.0_f64 ).abs() < PERCENT_TOLERANCE {
        // check for O(log(n)) -- (t2/t1) / (log(n*3)/log(n)) ~= 1
        computed_complexity = BigOAlgorithmComplexity::OLogN;
    } else if ( ((t2/t1) / 3.0_f64) - 1.0_f64 ).abs() < PERCENT_TOLERANCE {
        // check for O(n) -- (t2/t1) / 3 ~= 1
        computed_complexity = BigOAlgorithmComplexity::ON;
    } else if ( ((t2/t1) / 3.0_f64) - 1.0_f64 ) > PERCENT_TOLERANCE {
        // check for worse than O(n)
        computed_complexity = BigOAlgorithmComplexity::WorseThanON;
    } else {
        // by exclusion...
        computed_complexity = BigOAlgorithmComplexity::BetweenOLogNAndON;
    }

    let report = format!("'{}' set resizing algorithm measurements:\n\
                                 pass         Δt            Σn            t⁻\n\
                                 1) {:>12}  {:>12}  {:>12.3}\n\
                                 2) {:>12}  {:>12}  {:>12.3}\n\
                                 --> Algorithm Analysis: {}\n",
                                measurements.measurement_name,
                                measurements.pass_1_total_time, measurements.delta_set_size,   t1,
                                measurements.pass_2_total_time, measurements.delta_set_size*2, t2,
                                BigOAlgorithmComplexity::as_pretty_str(&computed_complexity));

    (computed_complexity, report)
}

mod tests {

    use super::super::{AlgorithmFnPtr,BigOAlgorithmType,TimeUnit,TimeUnits,run_pass};

    use super::*;
    use std::time::SystemTime;
    use std::io;
    use std::io::Write;
    use std::ops::Range;
    use std::convert::TryInto;

    use serial_test::serial;

    #[cfg(debug_assertions)]
    /// busy waiting constant for debug compilation
    const BUSY_LOOP_DELAY: u32 = 9999;

    #[cfg(not(debug_assertions))]
    /// busy waiting constant for release compilation
    const BUSY_LOOP_DELAY: u32 = 9999999;

    #[test]
    fn serialization() {
        println!("BigOAlgorithmComplexity enum members, as strings:");
        println!("\t{:?} => '{}'\n\
                  \t{:?} => '{}'\n\
                  \t{:?} => '{}'\n\
                  \t{:?} => '{}'\n\
                  \t{:?} => '{}'\n\
                  \t{:?} => '{}'",
                 BigOAlgorithmComplexity::BetterThanO1,      BigOAlgorithmComplexity::as_pretty_str(&BigOAlgorithmComplexity::BetterThanO1),
                 BigOAlgorithmComplexity::O1,                BigOAlgorithmComplexity::as_pretty_str(&BigOAlgorithmComplexity::O1),
                 BigOAlgorithmComplexity::OLogN,             BigOAlgorithmComplexity::as_pretty_str(&BigOAlgorithmComplexity::OLogN),
                 BigOAlgorithmComplexity::BetweenOLogNAndON, BigOAlgorithmComplexity::as_pretty_str(&BigOAlgorithmComplexity::BetweenOLogNAndON),
                 BigOAlgorithmComplexity::ON,                BigOAlgorithmComplexity::as_pretty_str(&BigOAlgorithmComplexity::ON),
                 BigOAlgorithmComplexity::WorseThanON,       BigOAlgorithmComplexity::as_pretty_str(&BigOAlgorithmComplexity::WorseThanON));
        println!();
    }

    #[test]
    fn analyse_constant_set_algorithm_theoretical_test() {

        let measurement_name = "analyse_constant_set_algorithm_theoretical_test";

        let assert = |measurement_name, expected_complexity, mut measurements: ConstantSetAlgorithmMeasurements| {
            measurements.measurement_name = measurement_name;
            let (observed_complexity, report) = analyse_constant_set_algorithm(&measurements);
            println!("{:?}\n{}", observed_complexity, report);
            assert_eq!(observed_complexity, expected_complexity, "Algorithm Analysis on CONSTANT SET algorithm for '{}' check failed!", measurement_name);
        };

        assert("Theoretical better than O(1) Update/Select", BigOAlgorithmComplexity::BetterThanO1, ConstantSetAlgorithmMeasurements {
            measurement_name,
            pass_1_total_time: 100,
            pass_2_total_time: 90,
            pass_1_set_size: 1000,
            pass_2_set_size: 2000,
            repetitions: 1000
        });

        assert("Theoretical O(1) Update/Select", BigOAlgorithmComplexity::O1, ConstantSetAlgorithmMeasurements {
            measurement_name,
            pass_1_total_time: 100,
            pass_2_total_time: 100,
            pass_1_set_size: 1000,
            pass_2_set_size: 2000,
            repetitions: 1000
        });

        assert("Theoretical O(log(n)) Update/Select", BigOAlgorithmComplexity::OLogN, ConstantSetAlgorithmMeasurements {
            measurement_name,
            pass_1_total_time: 100,
            pass_2_total_time: 111,
            pass_1_set_size: 1000,
            pass_2_set_size: 2000,
            repetitions: 1000
        });

        assert("Theoretical between O(log(n)) and O(n) Update/Select", BigOAlgorithmComplexity::BetweenOLogNAndON, ConstantSetAlgorithmMeasurements {
            measurement_name,
            pass_1_total_time: 100,
            pass_2_total_time: 200,
            pass_1_set_size: 1000,
            pass_2_set_size: 2500,
            repetitions: 1000
        });

        assert("Theoretical O(n) Update/Select", BigOAlgorithmComplexity::ON, ConstantSetAlgorithmMeasurements {
            measurement_name,
            pass_1_total_time: 100,
            pass_2_total_time: 200,
            pass_1_set_size: 1000,
            pass_2_set_size: 2000,
            repetitions: 1000
        });

        assert("Theoretical worse than O(n) Update/Select", BigOAlgorithmComplexity::WorseThanON, ConstantSetAlgorithmMeasurements {
            measurement_name,
            pass_1_total_time: 100,
            pass_2_total_time: 222,
            pass_1_set_size: 1000,
            pass_2_set_size: 2000,
            repetitions: 1000
        });

    }

    #[test]
    fn analyse_set_resizing_algorithm_theoretical_test() {

        let measurement_name = "analyse_set_resizing_algorithm_theoretical_test";

        let assert = |measurement_name, expected_complexity, mut measurements: SetResizingAlgorithmMeasurements| {
            measurements.measurement_name = measurement_name;
            let (observed_complexity, report) = analyse_set_resizing_algorithm(&measurements);
            println!("{:?}\n{}", observed_complexity, report);
            assert_eq!(observed_complexity, expected_complexity, "Algorithm Analysis on SET RESIZING algorithm for '{}' check failed!", measurement_name);
        };

        assert("Theoretical better than O(1) Insert/Delete", BigOAlgorithmComplexity::BetterThanO1, SetResizingAlgorithmMeasurements {
            measurement_name,
            pass_1_total_time: 100,
            pass_2_total_time: 90,
            delta_set_size: 1000
        });

        assert("Theoretical O(1) Insert/Delete", BigOAlgorithmComplexity::O1, SetResizingAlgorithmMeasurements {
            measurement_name,
            pass_1_total_time: 100,
            pass_2_total_time: 100,
            delta_set_size: 1000
        });

        assert("Theoretical O(log(n)) Insert/Delete", BigOAlgorithmComplexity::OLogN, SetResizingAlgorithmMeasurements {
            measurement_name,
            pass_1_total_time: 100,
            pass_2_total_time: 111,
            delta_set_size: 1000
        });

        assert("Theoretical between O(log(n)) and O(n) Insert/Delete", BigOAlgorithmComplexity::BetweenOLogNAndON, SetResizingAlgorithmMeasurements {
            measurement_name,
            pass_1_total_time: 100,
            pass_2_total_time: 200,
            delta_set_size: 1000
        });

        assert("Theoretical O(n) Insert/Delete", BigOAlgorithmComplexity::ON, SetResizingAlgorithmMeasurements {
            measurement_name,
            pass_1_total_time: 100,
            pass_2_total_time: 300,
            delta_set_size: 1000
        });

        assert("Theoretical worse than O(n) Insert/Delete", BigOAlgorithmComplexity::WorseThanON, SetResizingAlgorithmMeasurements {
            measurement_name,
            pass_1_total_time: 100,
            pass_2_total_time: 333,
            delta_set_size: 1000
        });
    }

    #[test]
    #[serial(cpu)]
    fn analyse_constant_set_algorithm_real_test() {

        fn o_1_select(mut _n: u32) -> u32 {
            busy_loop(BUSY_LOOP_DELAY*10)
        }

        fn o_log_n_select(mut n: u32) -> u32 {
            let mut r: u32 = 0;
            while n > 0 {
                r = r + busy_loop(BUSY_LOOP_DELAY*2);
                n = n/2;
            }
            r
        }

        fn o_n_select(mut n: u32) -> u32 {
            let mut r: u32 = 0;
            while n > 0 {
                r = r + busy_loop(BUSY_LOOP_DELAY/100);
                n = n-1;
            }
            r
        }

        let assert = |measurement_name, select_function: fn(u32) -> u32, expected_complexity| {
            let repetitions = 1000;
            let pass_1_set_size = repetitions;
            let pass_2_set_size = repetitions*3;
            print!("Real '{}' adding {} elements on each pass ", measurement_name, repetitions);

            let (_warmup_time,           r1) = _run_pass("(warmup: ", "", select_function, &BigOAlgorithmType::ConstantSet, 0..repetitions/10, &TimeUnits::MICROSECOND);
            let (pass_1_total_time, r2) = _run_pass("; pass1: ", "", select_function, &BigOAlgorithmType::ConstantSet, 0..pass_1_set_size, &TimeUnits::MICROSECOND);
            let (pass_2_total_time, r3) = _run_pass("; pass2: ", "): ", select_function, &BigOAlgorithmType::ConstantSet, pass_2_set_size-repetitions..pass_2_set_size, &TimeUnits::MICROSECOND);

            let (observed_complexity, report) = analyse_constant_set_algorithm(&ConstantSetAlgorithmMeasurements {
                measurement_name,
                pass_1_total_time,
                pass_2_total_time,
                pass_1_set_size,
                pass_2_set_size,
                repetitions,
            });
            println!("'{:?}' (r={})\n{}", observed_complexity, r1^r2^r3, report);
            assert_eq!(observed_complexity, expected_complexity, "Algorithm Analysis on CONSTANT SET algorithm for '{}' check failed!", measurement_name);

        };

        assert("O1_select() function", o_1_select, BigOAlgorithmComplexity::O1);
        assert("OLogN_select() function", o_log_n_select, BigOAlgorithmComplexity::OLogN);
        assert("ON_select() function", o_n_select, BigOAlgorithmComplexity::ON);

    }

    #[test]
    #[serial(cpu)]
    fn analyse_set_resizing_algorithm_real_test() {

        fn o_1_insert(mut _n: u32) -> u32 {
            busy_loop(BUSY_LOOP_DELAY*2)
        }

        fn o_log_n_insert(mut n: u32) -> u32 {
            let mut r: u32 = 0;
            while n > 0 {
                r = r ^ busy_loop(BUSY_LOOP_DELAY/2);
                n = n/2;
            }
            r
        }

        /// this would be an O(n/2) function -- the average case for a naive sorted insert... but still O(n). Change n = n-2 to n = n-1 and the analysis will be the same.
        fn o_n_insert(mut n: u32) -> u32 {
            let mut r: u32 = 0;
            while n > 1 {
                r = r ^ busy_loop(BUSY_LOOP_DELAY/100);
                n = n-2;
            }
            r
        }

        let assert = |measurement_name, insert_function: fn(u32) -> u32, expected_complexity| {
            let delta_set_size = 2000;
            print!("Real '{}' with {} elements on each pass ", measurement_name, delta_set_size);

            /* warmup pass -- container / database should be reset before and after this */
            let (_warmup_time,           r1) = _run_pass("(warmup: ", "", insert_function, &BigOAlgorithmType::SetResizing, 0..delta_set_size/10, &TimeUnits::MICROSECOND);
            /* if we were operating on real data, we would reset the container / database after the warmup, before running pass 1 */
            let (pass_1_total_time, r2) = _run_pass("; pass1: ", "", insert_function, &BigOAlgorithmType::SetResizing, 0..delta_set_size, &TimeUnits::MICROSECOND);
            let (pass_2_total_time, r3) = _run_pass("; pass2: ", "): ", insert_function, &BigOAlgorithmType::SetResizing, delta_set_size..delta_set_size*2, &TimeUnits::MICROSECOND);

            let (observed_complexity, report) = analyse_set_resizing_algorithm(&SetResizingAlgorithmMeasurements {
                measurement_name,
                pass_1_total_time,
                pass_2_total_time,
                delta_set_size,
            });
            println!("'{:?}' (r={})\n{}", observed_complexity, r1^r2^r3, report);
            assert_eq!(observed_complexity, expected_complexity, "Algorithm Analysis on SET RESIZING algorithm for '{}' check failed!", measurement_name);

        };

        assert("O1_insert() function", o_1_insert, BigOAlgorithmComplexity::O1);
        assert("OLogN_insert() function", o_log_n_insert, BigOAlgorithmComplexity::OLogN);
        assert("ON_insert() function", o_n_insert, BigOAlgorithmComplexity::ON);
    }

   #[inline]
    fn busy_loop(iterations: u32) -> u32 {
        let mut r: u32 = iterations;
        for i in 0..iterations {
            r ^= i;
        }
        r
    }

    /// wrap around the original 'run_pass' to output intermediate results
    fn _run_pass<T: TryInto<u64>>(result_prefix: &str, result_suffix: &str, algorithm: fn(u32) -> u32, algorithm_type: &BigOAlgorithmType, range: Range<u32>, unit: &TimeUnit<T>) -> (u64, u32) {
        io::stdout().flush().unwrap();
        let (pass_elapsed_us, r) = run_pass(algorithm, algorithm_type, range, unit);
        print!("{}{}{}{}", result_prefix, pass_elapsed_us, unit.unit_str, result_suffix);
        io::stdout().flush().unwrap();
        (pass_elapsed_us, r)
    }

}