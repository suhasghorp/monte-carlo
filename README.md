# monte-carlo
Rust Monte-Carlo Option Pricer

suhas@suhas-VirtualBox:~/rust-projects/monte-carlo$ cargo build --release
   Compiling monte-carlo v0.1.0 (/home/suhas/rust-projects/monte-carlo)
    Finished release [optimized + debuginfo] target(s) in 2.50s
suhas@suhas-VirtualBox:~/rust-projects/monte-carlo$ cargo run --release
    Finished release [optimized + debuginfo] target(s) in 0.03s
     Running `target/release/monte-carlo`
Non-parallel Price is 28.48201998899803, took 2.292222318s
Parallel Price is 28.488221293384285, took 613.905618ms
suhas@suhas-VirtualBox:~/rust-projects/monte-carlo$ 
