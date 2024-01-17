//! This is the main example for running this library as an executable, this does:
//! 1. Read AIG.
//! 2. Make Circuit from aig.
//! 3. Technology map the circuit.
//! 4. Make FiniteStateTransitionSystem from circuit.
//!
//! Release mode:
//! cargo run --release --package rust-formal-verification --example read_aig_and_prove_it
//! Elapsed time = 34.38172 seconds
//!
//! cargo run --package rust-formal-verification --example read_aig_and_prove_it

// ********************************************************************************************
// use
// ********************************************************************************************

use rust_formal_verification::{
    algorithms::proof::{
        long_trace_lemma_search::LongTraceLemmaSearchParameters,
        property_directed_reachability::PropertyDirectedReachabilityParameters,
        LongTraceLemmaSearch, ProofResult, PropertyDirectedReachability,
    },
    models::{AndInverterGraph, Circuit, FiniteStateTransitionSystem, LiteralWeights},
    solvers::sat::incremental::CaDiCalSolver,
};
use std::env;
use std::time;

// ********************************************************************************************
// enum
// ********************************************************************************************

pub enum Algorithm {
    LongTraceLemmaSearch,
    PropertyDirectedReachability,
}

// ********************************************************************************************
// parameters
// ********************************************************************************************

const TEST_ALGORITHM: Algorithm = Algorithm::LongTraceLemmaSearch;

// ********************************************************************************************
// run function
// ********************************************************************************************

fn run_file(aig_file_path: &str, extra_verbose: bool) {
    // mark start time
    let start_time = time::Instant::now();

    println!("aig_file_path = {}", aig_file_path);

    // read AIG
    let aig = AndInverterGraph::from_aig_path(aig_file_path).unwrap();

    // make circuit
    let mut circuit = Circuit::from_aig(&aig);

    // simplify circuit
    println!("Technology mapping...");
    circuit.simplify_circuit_before_using_proof_engine(true);
    println!("Finished technology mapping...");

    println!("Making Transition System...");
    let assume_output_is_bad = circuit.get_number_of_bad_wires() == 0;
    let mut fin_state =
        FiniteStateTransitionSystem::new(circuit.clone(), assume_output_is_bad).unwrap();
    println!("Finished making Transition System...");

    println!(
        "Number of state variables = {}",
        fin_state.get_state_variables().len()
    );
    println!(
        "Number of input variables = {}",
        fin_state.get_input_variables().len()
    );

    if extra_verbose && (circuit.get_highest_signal().get_number() < 10000) {
        println!("Printing circuit...");
        // let _ = _print_circuit(&circuit);
        println!("Finished printing circuit...");
    }

    let prove_result = match TEST_ALGORITHM {
        Algorithm::LongTraceLemmaSearch => {
            let mut parameters = LongTraceLemmaSearchParameters::new();
            parameters.print_lemmas = extra_verbose;
            let mut solver =
                LongTraceLemmaSearch::<CaDiCalSolver>::new(fin_state.to_owned(), parameters);
            solver.prove()
        }
        Algorithm::PropertyDirectedReachability => {
            let mut parameters = PropertyDirectedReachabilityParameters::new();
            if extra_verbose {
                parameters.should_use_internal_signal_generalization = false;
            }
            let mut solver = PropertyDirectedReachability::<CaDiCalSolver>::new(
                fin_state.to_owned(),
                parameters,
            );
            solver.prove()
        }
    };

    let duration = start_time.elapsed();

    // print time
    println!("Elapsed time = {}", duration.as_secs_f32());

    // print result
    match prove_result {
        ProofResult::Ok(p) => {
            println!("Safe, checking invariant.");
            fin_state.check_invariant::<rust_formal_verification::solvers::sat::incremental::CaDiCalSolver>(&p.invariant).unwrap();
            println!("Invariant check passed!");
            if extra_verbose {
                println!("Invariant = {}", p.invariant);
                let mut weights = LiteralWeights::new(&fin_state, 1, 0);
                for clause in p.invariant.iter() {
                    weights.update_weights_on_add(clause.iter());
                    // println!("weight = {}", weights.get_weight(clause));
                }
                weights.print_sorted_by_literal_weights(true);
                weights.print_sorted_by_variable_weight(true);
            }
        }
        ProofResult::Err(e) => {
            // do nothing for now
            println!("Unsafe, checking counter example.");
            fin_state.check_counter_example(e, extra_verbose).unwrap();
            println!("Counter example check passed.");
            // println!("Unsafe, depth = {depth}");
        }
    }
}

// ********************************************************************************************
// test functions
// ********************************************************************************************

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "USAGE: ./ltls <AIGER file>");
    // if args.len() == 2 {
    let aig_file_path = &args[1];
    run_file(aig_file_path, false)
    // }
}
