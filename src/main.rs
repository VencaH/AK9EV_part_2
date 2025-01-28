use std::error;
use std::env;
use decorum::cmp::CanonicalOrd;


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

fn get_row<T>(parameters: &BenchmarkParameters, problem: &T) -> Vec<(f32,f32)> 
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
    let mut sorted = result.clone();
    sorted.sort_by(| a, b| a.cmp_canonical(b));
    result.iter().map(|item| {
        let naive_position = sorted.iter().position(|x| x == item).unwrap_or(99usize);
        let rank  = &sorted.get(naive_position+1usize).and_then(|next| if &sorted[naive_position] == next {Some((naive_position as f32 + 3f32) / 2f32)} else {None}).unwrap_or(naive_position as f32 + 1f32);
        (*item,*rank)
    }).collect::<Vec<(f32,f32)>>()
}

fn get_stats(dim: usize) -> Result<Vec<Vec<(f32,f32)>>, Box<dyn std::error::Error>> 
{
   let parameters = BenchmarkParameters::new(dim);

    let ackley = build_problem::<ackley::Ackley>(parameters.dim)?; 
    let alpine2 = alpine2::Alpine2::builder().minimum(0f32).maximum(100f32).dimensions(2).build()? ; 
    let deb1 = build_problem::<deb1::Deb1>(parameters.dim)?; 
    let foth_dejong = build_problem::<foth_dejong::FothDejong>(parameters.dim)?; 
    let griewank = build_problem::<griewank::Griewank>(parameters.dim)?; 
    let michalewicz = build_problem::<michalewicz::Michalewicz>(parameters.dim)?; 
    let periodic = build_problem::<periodic::Periodic>(parameters.dim)?; 
    let qinq = build_problem::<qing::Qing>(parameters.dim)?; 
    let quintic = build_problem::<quintic::Quintic>(parameters.dim)?; 
    let rastrigin = build_problem::<rastrigin::Rastrigin>(parameters.dim)?; 
    let salomon = build_problem::<salomon::Salomon>(parameters.dim)?; 
    let schwefel = build_problem::<schwefel::Schwefel>(parameters.dim)?; 
    let styblinsky = build_problem::<styblinsky_and_tang::StyblinskyAndTang>(parameters.dim)?; 
    let trd_dejong = build_problem::<trd_dejong::TrdDejong>(parameters.dim)?; 
    let xinsheyang = build_problem::<xinsheyang::XinSheYang>(parameters.dim)?; 

    let mut result =Vec::new();
    //println!("Calculating Ackley");
    result.push(get_row(&parameters, &ackley));
    //println!("Calculating Alpine2");
    result.push(get_row(&parameters, &alpine2));
    //println!("Calculating Deb1");
    result.push(get_row(&parameters, &deb1));
    //println!("Calculating Forth DeJong");
    result.push(get_row(&parameters, &foth_dejong));
    //println!("Calculating Griewank");
    result.push(get_row(&parameters, &griewank));
    //println!("Calculating Michalewicz");
    result.push(get_row(&parameters, &michalewicz));
    //println!("Calculating Periodic");
    result.push(get_row(&parameters, &periodic));
    //println!("Calculating Qinq");
    result.push(get_row(&parameters, &qinq));
    //println!("Calculating Quintic");
    result.push(get_row(&parameters, &quintic));
    //println!("Calculating Rastrigin");
    result.push(get_row(&parameters, &rastrigin));
    //println!("Calculating Salomon");
    result.push(get_row(&parameters, &salomon));
    //println!("Calculating Schwefel");
    result.push(get_row(&parameters, &schwefel));
    //println!("Calculating Styblinsky");
    result.push(get_row(&parameters, &styblinsky));
    //println!("Calculating Trd_dejong");
    result.push(get_row(&parameters, &trd_dejong));
    //println!("Calculating XinSheYang");
    result.push(get_row(&parameters, &xinsheyang));

    Ok(result)
}

fn friedman(input: &Vec<Vec<(f32,f32)>>) -> (f32, bool) {
    let n = 15f32;
    let k = 3f32;
    let chi_critical = 23.685f32;
    let sums =input.into_iter().fold( [0f32,0f32,0f32], |mut acc, row| {
       acc[0] = acc[0] +  row[0].1 ;
       acc[1] = acc[1] +  row[1].1 ;
       acc[2] = acc[2] +  row[2].1 ;
       acc
    });
    let chi = (12f32/ (n * k * (k + 1f32))) * (sums.into_iter().fold(0f32, |acc, item| {acc + (item.powf(2f32) as f32)} )) - (3f32 * n * (k + 1f32));
    (chi, chi > chi_critical)
}


fn main() -> Result<(), Box<dyn error::Error>> {
    let dim = env::args().collect::<Vec<String>>()[1].parse::<usize>().expect("Error: incorect number of dimensions");
    let result = get_stats(dim)?;
    //result.iter().for_each(|row| println!("{row:?}"));
    print!("{:?}",friedman(&result));
    Ok(())
}
