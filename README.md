# ReedSolomonRacing

Testing on a M3 Max 128GB:

### Go:

```
Small data reconstruction:
Encoding 16 KB of data with 4+2 encoding...
Encoding time: 6.458µs
Reconstructing 2 missing shards...
Reconstruction time: 8.25µs
Verification successful!

Large data reconstruction:
Encoding 64 MB of data with 4+2 encoding...
Encoding time: 2.650875ms
Reconstructing 2 missing shards...
Reconstruction time: 3.517417ms
Verification successful!
```

I noticed with Go that it starts running slow a few times, then gets very fast. Maybe that's some CPU caching?

```
Small data reconstruction:
Encoding 16 KB of data with 4+2 encoding...
Encoding time: 57.541µs
Reconstructing 2 missing shards...
Reconstruction time: 64µs
Verification successful!

Small data reconstruction:
Encoding 16 KB of data with 4+2 encoding...
Encoding time: 74.958µs
Reconstructing 2 missing shards...
Reconstruction time: 60.75µs
Verification successful!

Small data reconstruction:
Encoding 16 KB of data with 4+2 encoding...
Encoding time: 6.458µs
Reconstructing 2 missing shards...
Reconstruction time: 8.25µs
Verification successful!
```




### Rust:
```
Small data reconstruction:
Encoding 16 KB of data with 4+2 encoding...
Encoding time: 20.167µs
Reconstructing 2 missing shards...
Reconstruction time: 27.833µs
Verification successful!

Large data reconstruction:
Encoding 64 MB of data with 4+2 encoding...
Encoding time: 10.878625ms
Reconstructing 2 missing shards...
Reconstruction time: 9.654167ms
Verification successful!

XOR small data reconstruction:
XOR encoding time: 667ns
XOR reconstruction time: 458ns
XOR verification successful!

XOR large data reconstruction:
XOR encoding time: 2.477792ms
XOR reconstruction time: 1.04ms
XOR verification successful!
```
