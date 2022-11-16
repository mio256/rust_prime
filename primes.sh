echo -- limit:1_000_000
echo -- rust
cargo build --release; cargo run
echo -- python
python3 prime.py
echo -- c
gcc prime.c; ./a.out