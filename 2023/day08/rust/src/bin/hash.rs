use std::collections::HashMap as StdHashMap;
use hashbrown::HashMap as HbHashMap;
use ahash::AHashMap;
use std::time::Instant;
use micromap::Map as MicroMap;

fn main() {
    const NUM_OF_ELEMENTS: usize = 6;
    let mut std_map = StdHashMap::new();
    let mut hb_map = HbHashMap::new();
    let mut a_map = AHashMap::new();
    let mut m_map: MicroMap<usize, usize, { NUM_OF_ELEMENTS }> = MicroMap::new();

    // Benchmark standard library's HashMap
    let start = Instant::now();
    for i in 0..NUM_OF_ELEMENTS {
        std_map.insert(i, i);
    }
    for i in 0..NUM_OF_ELEMENTS {
        let _ = std_map.get(&i);
    }
    let duration_std = start.elapsed();

    // Benchmark hashbrown's HashMap
    let start = Instant::now();
    for i in 0..NUM_OF_ELEMENTS {
        hb_map.insert(i, i);
    }
    for i in 0..NUM_OF_ELEMENTS {
        let _ = hb_map.get(&i);
    }
    let duration_hb = start.elapsed();

    // Benchmark ahash's HashMap
    let start = Instant::now();
    for i in 0..NUM_OF_ELEMENTS {
        a_map.insert(i, i);
    }
    for i in 0..NUM_OF_ELEMENTS {
        let _ = a_map.get(&i);
    }
    let duration_ah = start.elapsed();

    // Benchmark micromap's HashMap
    let start = Instant::now();
    for i in 0..NUM_OF_ELEMENTS {
        m_map.insert(i, i);
    }
    for i in 0..NUM_OF_ELEMENTS {
        let _ = m_map.get(&i);
    }
    let duration_m = start.elapsed();

    println!("Std HashMap: {:?}", duration_std);
    println!("Hashbrown HashMap: {:?}", duration_hb);
    println!("AHash HashMap: {:?}", duration_ah);
    println!("MicroMap HashMap: {:?}", duration_m);
}