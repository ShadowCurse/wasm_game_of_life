#![feature(test)]

extern crate test;
extern crate wasm_game_of_life;

#[bench]
fn universe_tick(b: &mut test::Bencher) {
    let mut universe = wasm_game_of_life::Universe::new(64, 64);
    b.iter(|| {
        universe.tick();
    });
}
