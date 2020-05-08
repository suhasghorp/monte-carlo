mod epg;
mod pricer;
use std::time::Instant;

fn main() {
    let mut start = Instant::now();
    let mut price = pricer::MCEuroOptPricer::new(200.0,200.0,0.15,0.1,1.0,pricer::OptionType::Call,365,1000000,false,0, 1).compute_price();
    let mut duration = start.elapsed();
    println!("Non-parallel Option Price is {}, took {:?}", price, duration);

    println!("Number of logical cores is {}", num_cpus::get());
    println!("Number of Scenarios: 1,000,000");
    start = Instant::now();
    price = pricer::MCEuroOptPricer::new(200.0,200.0,0.15,0.1,1.0,pricer::OptionType::Call,365,1000000,true,0, 1).compute_price();
    duration = start.elapsed();
    println!("Parallel Option Price is {}, took {:?}", price, duration);
}

