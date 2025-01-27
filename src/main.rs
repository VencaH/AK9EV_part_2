use std::error;

use heuristics::{
    benchmarks::traits::{HasBuilder, Benchmark, BuilderError},
    benchmarks::*,
    evol_arg::{
        de::{De, Strategy, Variant},
        pso::Pso,
    },
    problem_definitions::ProblemDomain,
};

struct BenchmarkParameters {
    dim: usize,
    max_cf: i32,
    pop: usize,
    f_rnd: f32,
    f_bst: f32,
    cr_rnd: f32,
    cr_bst: f32,
    c: f32,
    w: f32,
}

impl BenchmarkParameters {
    fn new(dim: usize) -> Self {
    let (f_rnd, cr_rnd) = (0.8, 0.9);
    let (f_bst, cr_bst) = (0.5, 0.9);
    let (c, w) = (1.49618, 0.7298);
    let (pop, max_cf) = match dim {
        2 => (10usize, 4000),
        10 => (20usize, 20000),
        _ => (40usize, 40000),
    };
        Self {
            dim,
            max_cf,
            pop,
            f_bst,
            f_rnd,
            cr_bst,
            cr_rnd,
            c,
            w,        }
    }
}



fn build_problem<T>(dim: usize) -> Result<T, BuilderError> 
where T: ProblemDomain + HasBuilder<T> + std::default::Default + Benchmark
{
    T::builder()
        .maximum(100f32)
        .minimum(-100f32)
        .dimensions(dim)
        .build()
}

fn get_row<T>(parameters: &BenchmarkParameters, problem: &T) -> Vec<f32> 
where T: Benchmark + ProblemDomain<Item = f32> + HasBuilder<T> + std::default::Default
{ 
    let mut result = Vec::new();
    result.push(
        (0..20)
            .filter_map(|_| {
                let mut solver = De::new(
                    &Variant::Rnd,
                    1,
                    Strategy::Bin,
                    parameters.max_cf,
                    parameters.pop,
                    parameters.f_rnd,
                    parameters.cr_rnd,
                    problem,
                );
                solver.run();
                solver.get_best().map(|m| m.get_cost())
            })
            .enumerate()
            .fold(0f32, |acc, (i, value)| {
                ((acc * i as f32) / (i as f32 + 1f32)) + value / (i as f32 + 1f32)
            }),
    );
    result.push(
        (0..20)
            .filter_map(|_| {
                let mut solver = De::new(
                    &Variant::Best,
                    1,
                    Strategy::Bin,
                    parameters.max_cf,
                    10,
                    parameters.f_bst,
                    parameters.cr_bst,
                    problem,
                );
                solver.run();
                solver.get_best().map(|m| m.get_cost())
            })
            .enumerate()
            .fold(0f32, |acc, (i, value)| {
                ((acc * i as f32) / (i as f32 + 1f32)) + value / (i as f32 + 1f32)
            }),
    );
    result.push(
        (0..20)
            .filter_map(|_| {
                let mut solver = Pso::new(parameters.max_cf, parameters.pop, parameters.w, parameters.c, parameters.c, problem);
                solver.run();
                solver.get_best()
            })
            .enumerate()
            .fold(0f32, |acc, (i, value)| {
                ((acc * i as f32) / (i as f32 + 1f32)) + value / (i as f32 + 1f32)
            }),
    );
    result
}

fn get_stats(dim: usize) -> Result<Vec<Vec<f32>>, Box<dyn std::error::Error>> 
{
   let parameters = BenchmarkParameters::new(2);

    let ackley = build_problem::<ackley::Ackley>(20)?; 
    let alpine2 = alpine2::Alpine2::builder().minimum(0f32).maximum(100f32).dimensions(20).build()? ; 
    let deb1 = build_problem::<deb1::Deb1>(20)?; 
    let foth_dejong = build_problem::<foth_dejong::FothDejong>(20)?; 
    let mut result =Vec::new();
    result.push(get_row(&parameters, &ackley));
    result.push(get_row(&parameters, &alpine2));
    result.push(get_row(&parameters, &deb1));
    result.push(get_row(&parameters, &foth_dejong));
    Ok(result)
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let result = get_stats(2)?;
    println!("Results: {:?}", result);
    Ok(())
}
