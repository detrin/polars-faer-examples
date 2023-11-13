# polars-faer-examples

## How to run
```bash
cargo run -- --train_input ./data/data_f10_s1000_train.csv --test_input ./data/data_f10_s1000_test.csv 
cargo build --release
./target/release/polars_faer_examples --train_input ./data/data_f10_s1000_train.csv --test_input ./data/data_f10_s1000_test.csv 
./benchmark.sh
```
