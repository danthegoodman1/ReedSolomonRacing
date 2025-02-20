package main

import (
	"crypto/rand"
	"fmt"
	"log"
	"time"

	"github.com/klauspost/reedsolomon"
)

func main() {
	fmt.Println("Shard by shard reconstruction:")
	shardByShard()
	fmt.Println("\nAt once reconstruction:")
	atOnce()
	fmt.Println("\nLarge data reconstruction:")
	largeData()
}

func shardByShard() {
	// Create encoder with 3 data shards and 2 parity shards
	enc, err := reedsolomon.New(3, 2)
	if err != nil {
		log.Fatal(err)
	}

	// Create the original data
	originalData := [][]byte{
		{0, 1, 2, 3},
		{4, 5, 6, 7},
		{8, 9, 10, 11},
		{0, 0, 0, 0}, // Parity shards
		{0, 0, 0, 0},
	}

	// Encode progressively
	err = enc.EncodeIdx(originalData[0], 0, originalData[3:])
	if err != nil {
		log.Fatal(err)
	}
	err = enc.EncodeIdx(originalData[1], 1, originalData[3:])
	if err != nil {
		log.Fatal(err)
	}
	err = enc.EncodeIdx(originalData[2], 2, originalData[3:])
	if err != nil {
		log.Fatal(err)
	}

	// Make a copy for reconstruction
	shards := make([][]byte, len(originalData))
	copy(shards, originalData)

	// Simulate loss of shards
	shards[0] = nil
	shards[4] = nil

	start := time.Now()
	err = enc.Reconstruct(shards)
	elapsed := time.Since(start)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Reconstruction time: %v\n", elapsed)

	// Verify the reconstruction
	ok, err := enc.Verify(shards)
	if err != nil {
		log.Fatal(err)
	}
	if !ok {
		log.Fatal("Verification failed")
	}
}

func atOnce() {
	// Create encoder with 3 data shards and 2 parity shards
	enc, err := reedsolomon.New(3, 2)
	if err != nil {
		log.Fatal(err)
	}

	// Create the original data
	originalData := [][]byte{
		{0, 1, 2, 3},
		{4, 5, 6, 7},
		{8, 9, 10, 11},
		{0, 0, 0, 0}, // Parity shards
		{0, 0, 0, 0},
	}

	// Encode all at once
	err = enc.Encode(originalData)
	if err != nil {
		log.Fatal(err)
	}

	// Make a copy for reconstruction
	shards := make([][]byte, len(originalData))
	copy(shards, originalData)

	// Simulate loss of shards
	shards[0] = nil
	shards[4] = nil

	start := time.Now()
	err = enc.Reconstruct(shards)
	elapsed := time.Since(start)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Reconstruction time: %v\n", elapsed)

	// Verify the reconstruction
	ok, err := enc.Verify(shards)
	if err != nil {
		log.Fatal(err)
	}
	if !ok {
		log.Fatal("Verification failed")
	}
}

func largeData() {
	const totalSize = 64 * 1024 * 1024 // 64MB
	const shardSize = totalSize / 4    // Split across 4 data shards

	// Create encoder with 4 data shards and 2 parity shards
	enc, err := reedsolomon.New(4, 2)
	if err != nil {
		log.Fatal(err)
	}

	// Generate random data for each shard
	originalData := make([][]byte, 6)
	for i := 0; i < 4; i++ { // Data shards
		originalData[i] = make([]byte, shardSize)
		_, err := rand.Read(originalData[i])
		if err != nil {
			log.Fatal(err)
		}
	}
	// Initialize parity shards
	for i := 4; i < 6; i++ {
		originalData[i] = make([]byte, shardSize)
	}

	fmt.Printf("Encoding %d MB of data with 4+2 encoding...\n", totalSize/1024/1024)
	start := time.Now()
	err = enc.Encode(originalData)
	encodeTime := time.Since(start)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Encoding time: %v\n", encodeTime)

	// Make a copy for reconstruction
	shards := make([][]byte, len(originalData))
	copy(shards, originalData)

	// Simulate loss of shards
	shards[0] = nil
	shards[4] = nil

	fmt.Println("Reconstructing 2 missing shards...")
	start = time.Now()
	err = enc.Reconstruct(shards)
	reconstructTime := time.Since(start)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Reconstruction time: %v\n", reconstructTime)

	// Verify the reconstruction
	ok, err := enc.Verify(shards)
	if err != nil {
		log.Fatal(err)
	}
	if !ok {
		log.Fatal("Verification failed")
	}
	fmt.Println("Verification successful!")
}
