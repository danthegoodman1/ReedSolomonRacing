# ReedSolomonRacing

Testing on a M3 Max 128GB:

Go:
```
Shard by shard reconstruction:
Reconstruction time: 12.5µs

At once reconstruction:
Reconstruction time: 1µs

Large data reconstruction:
Encoding 64 MB of data with 4+2 encoding...
Encoding time: 2.463958ms
Reconstructing 2 missing shards...
Reconstruction time: 3.146042ms
Verification successful!
```


Rust:
```
Shard by shard reconstruction:
Reconstruction time: 10.459µs

At once reconstruction:
Reconstruction time: 6.292µs

Large data reconstruction:
Encoding 64 MB of data with 4+2 encoding...
Encoding time: 11.832458ms
Reconstructing 2 missing shards...
Reconstruction time: 10.296875ms
Verification successful!

XOR large data reconstruction:
XOR encoding time: 3.448125ms
XOR reconstruction time: 1.62125ms
XOR verification successful!
```
