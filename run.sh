cd solution
cargo build --release
cd ..
../../linux_game_engine -f ../../maps/map01 -p1 solution/target/release/solution -p2 ../../linux_robots/bender