use std::env;
use std::process;
extern crate concurrent_prime_sieve;
use concurrent_prime_sieve::filter::prime_filter;
mod draw_spiral;
fn help() -> !{
    println!("Can only use up to 2 environment variables. E.G.");
    println!(">>>cargo run <size> <write_to_path>");
    process::exit(0);
}
fn get_size_from_env(env_args: &Vec<String>, default_size: usize) -> usize {
    match env_args[1].trim().parse(){
        Ok(n) => n,
        Err(_) => default_size,
    }
}
fn get_env(default_size: usize) -> (String, usize){
    let env_args: Vec<String> = env::args().collect();
    let (path, size) = match env_args.len(){
        1 => ("", default_size),
        2 => ("", get_size_from_env(&env_args, default_size)),
        3 => (env_args[2].as_str(), get_size_from_env(&env_args, default_size)),
        _ => help(),
    };
    (String::from(path), size)
}
fn main() {
    let (path, size) = get_env(200);
    // println!("{}", path);
    let num_primes = size*size + 1;
    let is_prime_iter = prime_filter(num_primes);
    draw_spiral::prime_filter_to_spiral_png(size, is_prime_iter, path);
    // for (i, j) in is_prime_iter.iter().enumerate(){
    //     println!("{} is prime: {}", i, j);
    // }
}
