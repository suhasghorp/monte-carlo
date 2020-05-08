mod epg;
mod pricer;
use std::time::Instant;

fn main() {
    let mut start = Instant::now();
    let mut price = pricer::MCEuroOptPricer::new(200.0,200.0,0.15,0.1,1.0,pricer::OptionType::Call,365,100000,false,0, 1).compute_price();
    let mut duration = start.elapsed();
    println!("Non-parallel Price is {}, took {:?}", price, duration);

    println!("Number of logical cores is {}", num_cpus::get());
    start = Instant::now();
    price = pricer::MCEuroOptPricer::new(200.0,200.0,0.15,0.1,1.0,pricer::OptionType::Call,365,100000,true,0, 1).compute_price();
    duration = start.elapsed();
    println!("Parallel Price is {}, took {:?}", price, duration);
}

