use reed_solomon_erasure::galois_8::{ReedSolomon, ShardByShard};
// or use the following for Galois 2^16 backend
// use reed_solomon_erasure::galois_16::ReedSolomon;
use std::time::Instant;
use rand::Rng;

fn main() {
    println!("Small data reconstruction:");
    small_data();
    println!("\nLarge data reconstruction:");
    large_data();
    println!("\nXOR small data reconstruction:");
    xor_small_data();
    println!("\nXOR large data reconstruction:");
    xor_large_data();
}



fn small_data() {
    // Calculate shard size
    const TOTAL_SIZE: usize = 1024 * 16; // 16KB in bytes
    const SHARD_SIZE: usize = TOTAL_SIZE / 4;    // Split across 4 data shards
    
    // Create Reed-Solomon encoder with 4 data shards and 2 parity shards
    let r = ReedSolomon::new(4, 2).unwrap(); // 4 data shards, 2 parity shards

    // Generate random data for each shard
    let mut rng = rand::thread_rng();
    let mut original_data: Vec<Vec<u8>> = vec![
        (0..SHARD_SIZE).map(|_| rng.gen()).collect(), // Data shard 1
        (0..SHARD_SIZE).map(|_| rng.gen()).collect(), // Data shard 2
        (0..SHARD_SIZE).map(|_| rng.gen()).collect(), // Data shard 3
        (0..SHARD_SIZE).map(|_| rng.gen()).collect(), // Data shard 4
        vec![0; SHARD_SIZE],                          // Parity shard 1
        vec![0; SHARD_SIZE],                          // Parity shard 2
    ];

    println!("Encoding {} KB of data with 4+2 encoding...", TOTAL_SIZE / 1024);
    let encode_timer = Instant::now();
    // Encode to generate parity shards
    r.encode(&mut original_data).unwrap();
    let encode_time = encode_timer.elapsed();
    println!("Encoding time: {:?}", encode_time);

    // Create reconstruction scenario
    let mut shards: Vec<_> = original_data.iter().cloned().map(Some).collect();
    
    // Remove two shards (can remove up to 2 with this configuration)
    shards[0] = None;
    shards[4] = None;

    println!("Reconstructing 2 missing shards...");
    let reconstruct_timer = Instant::now();
    // Reconstruct missing shards
    r.reconstruct(&mut shards).unwrap();
    let reconstruct_time = reconstruct_timer.elapsed();
    println!("Reconstruction time: {:?}", reconstruct_time);

    // Convert back to normal shard arrangement
    let result: Vec<_> = shards.into_iter().filter_map(|x| x).collect();

    // Verify the reconstruction
    assert!(r.verify(&result).unwrap());
    assert_eq!(original_data, result);
    println!("Verification successful!");
}

fn large_data() {
    // Calculate shard size
    const TOTAL_SIZE: usize = 64 * 1024 * 1024; // 64MB in bytes
    const SHARD_SIZE: usize = TOTAL_SIZE / 4;    // Split across 4 data shards
    
    // Create Reed-Solomon encoder with 4 data shards and 2 parity shards
    let r = ReedSolomon::new(4, 2).unwrap(); // 4 data shards, 2 parity shards

    // Generate random data for each shard
    let mut rng = rand::thread_rng();
    let mut original_data: Vec<Vec<u8>> = vec![
        (0..SHARD_SIZE).map(|_| rng.gen()).collect(), // Data shard 1
        (0..SHARD_SIZE).map(|_| rng.gen()).collect(), // Data shard 2
        (0..SHARD_SIZE).map(|_| rng.gen()).collect(), // Data shard 3
        (0..SHARD_SIZE).map(|_| rng.gen()).collect(), // Data shard 4
        vec![0; SHARD_SIZE],                          // Parity shard 1
        vec![0; SHARD_SIZE],                          // Parity shard 2
    ];

    println!("Encoding {} MB of data with 4+2 encoding...", TOTAL_SIZE / 1024 / 1024);
    let encode_timer = Instant::now();
    // Encode to generate parity shards
    r.encode(&mut original_data).unwrap();
    let encode_time = encode_timer.elapsed();
    println!("Encoding time: {:?}", encode_time);

    // Create reconstruction scenario
    let mut shards: Vec<_> = original_data.iter().cloned().map(Some).collect();
    
    // Remove two shards (can remove up to 2 with this configuration)
    shards[0] = None;
    shards[4] = None;

    println!("Reconstructing 2 missing shards...");
    let reconstruct_timer = Instant::now();
    // Reconstruct missing shards
    r.reconstruct(&mut shards).unwrap();
    let reconstruct_time = reconstruct_timer.elapsed();
    println!("Reconstruction time: {:?}", reconstruct_time);

    // Convert back to normal shard arrangement
    let result: Vec<_> = shards.into_iter().filter_map(|x| x).collect();

    // Verify the reconstruction
    assert!(r.verify(&result).unwrap());
    assert_eq!(original_data, result);
    println!("Verification successful!");
}

fn xor_small_data() {
    // Use 16 KB of total data split equally among 4 data blocks.
    const TOTAL_SIZE: usize = 1024 * 16; // 16 KB total
    const NUM_DATA_BLOCKS: usize = 4;           // 4 data blocks
    let block_size = TOTAL_SIZE / NUM_DATA_BLOCKS;

    let mut rng = rand::thread_rng();
    // Generate 4 data blocks filled with random bytes.
    let data_blocks: Vec<Vec<u8>> = (0..NUM_DATA_BLOCKS)
        .map(|_| (0..block_size).map(|_| rng.gen()).collect())
        .collect();

    // Calculate the EC block as the XOR of all data blocks.
    let encode_timer = Instant::now();
    let mut ec_block = vec![0u8; block_size];
    for block in &data_blocks {
        for (i, byte) in block.iter().enumerate() {
            ec_block[i] ^= byte;
        }
    }
    let encode_time = encode_timer.elapsed();
    println!("XOR encoding time: {:?}", encode_time);

    // Simulate the loss of a single block (for instance, block index 2).
    let lost_index = 2;
    let original_lost_block = data_blocks[lost_index].clone();

    // Create a vector of Option blocks like with Reed–Solomon and remove the lost block.
    let mut shards: Vec<Option<Vec<u8>>> = data_blocks.into_iter().map(Some).collect();
    shards[lost_index] = None;

    // Reconstruct the missing block.
    // Since ec_block is the XOR of all original data blocks, we have:
    //
    //   ec_block = block0 ⊕ block1 ⊕ block2 ⊕ block3
    //
    // If one block is missing (say block2), then:
    //
    //   block2 = ec_block ⊕ block0 ⊕ block1 ⊕ block3
    //
    // Here we start with the parity block and then XOR in the available blocks.
    let mut recovered = ec_block.clone();
    let reconstruct_timer = Instant::now();
    for shard in &shards {
        if let Some(block) = shard {
            for (i, byte) in block.iter().enumerate() {
                recovered[i] ^= byte;
            }
        }
    }
    let reconstruct_time = reconstruct_timer.elapsed();
    println!("XOR reconstruction time: {:?}", reconstruct_time);

    // Verify that the recovered block matches the original lost block.
    assert_eq!(original_lost_block, recovered);
    println!("XOR verification successful!");
}

fn xor_large_data() {
    // Use 64 MB of total data split equally among 4 data blocks.
    const TOTAL_SIZE: usize = 64 * 1024 * 1024; // 64 MB total
    const NUM_DATA_BLOCKS: usize = 4;           // 4 data blocks
    let block_size = TOTAL_SIZE / NUM_DATA_BLOCKS;

    let mut rng = rand::thread_rng();
    // Generate 4 data blocks filled with random bytes.
    let data_blocks: Vec<Vec<u8>> = (0..NUM_DATA_BLOCKS)
        .map(|_| (0..block_size).map(|_| rng.gen()).collect())
        .collect();

    // Calculate the EC block as the XOR of all data blocks.
    let encode_timer = Instant::now();
    let mut ec_block = vec![0u8; block_size];
    for block in &data_blocks {
        for (i, byte) in block.iter().enumerate() {
            ec_block[i] ^= byte;
        }
    }
    let encode_time = encode_timer.elapsed();
    println!("XOR encoding time: {:?}", encode_time);

    // Simulate the loss of a single block (for instance, block index 2).
    let lost_index = 2;
    let original_lost_block = data_blocks[lost_index].clone();

    // Create a vector of Option blocks like with Reed–Solomon and remove the lost block.
    let mut shards: Vec<Option<Vec<u8>>> = data_blocks.into_iter().map(Some).collect();
    shards[lost_index] = None;

    // Reconstruct the missing block.
    // Since ec_block is the XOR of all original data blocks, we have:
    //
    //   ec_block = block0 ⊕ block1 ⊕ block2 ⊕ block3
    //
    // If one block is missing (say block2), then:
    //
    //   block2 = ec_block ⊕ block0 ⊕ block1 ⊕ block3
    //
    // Here we start with the parity block and then XOR in the available blocks.
    let mut recovered = ec_block.clone();
    let reconstruct_timer = Instant::now();
    for shard in &shards {
        if let Some(block) = shard {
            for (i, byte) in block.iter().enumerate() {
                recovered[i] ^= byte;
            }
        }
    }
    let reconstruct_time = reconstruct_timer.elapsed();
    println!("XOR reconstruction time: {:?}", reconstruct_time);

    // Verify that the recovered block matches the original lost block.
    assert_eq!(original_lost_block, recovered);
    println!("XOR verification successful!");
}
