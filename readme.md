# Project server-client app (RUST) with runetime reloadable WebAssembly modules and its performance evaluation

## Overview

This is a draft project which main purpose is to provide an experience
working with various aspects of Rust language and WASM technology ecosystem and tools.
Final goal of this project is to evaluate performance of code being executed on native
platform and inside WASM runtimes(wasm3,wamtime) and demonstate possibility of runtime wasm module switching during runetime.
During project work I tried to utilize the most popular and usefull crates that can be applied to
this work (tokio,tonic,redis etc..), also, investigating WASM runtimes and toolchain.

### Code structure
```console
===============================================================================
 Language            Files        Lines         Code     Comments       Blanks
===============================================================================
 Makefile                1           18           13            2            3
 Protocol Buffers        1           36           27            2            7
 Plain Text              1          201            0          169           32
 TOML                   19          497          444            4           49
 YAML                    1           13           13            0            0
-------------------------------------------------------------------------------
 Markdown                5          301            0          217           84
 |- BASH                 1           14           14            0            0
 |- Rust                 4           93           68            4           21
 (Total)                            408           82          221          105
-------------------------------------------------------------------------------
 Rust                   51         4167         3681          154          332
 |- Markdown            19          247            0          222           25
 (Total)                           4414         3681          376          357
===============================================================================
 Total                  79         5233         4178          548          507
===============================================================================
```

## Motivation

WASM technology is a very interesting and perspective technology
which main advantages are: sandboxing, planform-independent bytecode, code optimization provided by LLVM, small binary size.
WASM technology, along with JS, may be used as efficent CPU-bound task executor to provide almost native code execution performance.

## Main technologies and covered tecniques
* [WASM]( https://webassembly.org/ )
* WASM runtimes and tools ([wasm3]( https://github.com/wasm3/wasm3 ), [wasmtime]( https://wasmtime.dev/ ), etc..)
* WASM RPC ([waPC]( https://wapc.io/ ))
* [Cryptography]( https://crates.io/crates/k256 )
* Rust Unsafe
* gRPC & protobuf (via tonic & prost)
* NoSQL (Redis)
* Tokio
* Shared memory
* Serialization/Deserialzation & bin/hex code streams

## Requirements
+ rustc 1.70.0-nightly
+ docker & docker-compose (or redis installed)
+ protobuf
+ wasm32-unknown-unknown rustup target installed

## Usage
1. Run `` docker-compose -f docker-compose-redis.yml up `` in redis-compose directory.
1. Setup desired messages number in file ``shmem-structs/src/lib.rs`` by adjusting ``MESSAGES_NUMBER`` value.
3. Compile WASM module `` cargo build -p module4-verify --target wasm32-unknown-unknown --release ``
4. Compile WASM module `` cargo build -p module6-verify --target wasm32-unknown-unknown --release ``
5. Run `` cargo run -p server --release ``
6. Run `` cargo run -p main-app --release -- --runner 0 --method 2 `` to run example with module runtime replace

## Project structure
![](https://github.com/Kerosin3/RUST-WASM-RPC-PROJECT/blob/main/docs/shema.jpg)

## Project organization
```bash
├── bins-----------------pre compiled executables
├── Cargo.toml
├── docs
├── interconnect---------common structeres between WASM and RUST
├── main-app-------------main application, generates messages and reads back from signer from shared memory
├── memshare-------------(being created during run) shared memory file
├── modules--------------wasm modules sources
├── proto----------------protobuf template
├── readme.md
├── redis-compose--------redis image
├── rust-toolchain.toml
├── server---------------server that signs messages which have been prefetched from redis, writes to shared memory
├── shmem-structs--------common data struct used by shared memory package
└── wasm-core------------WASM core library
```
# Results

## Hardware Platform
* x86_64, Intel(R) Core(TM) i7-4770K CPU @ 8 cores @ 3.50GHz 
* Linux@Windows-WSL x86_64

### [SCHNORR] 
 * 1024 messages, signing method: [Schnorr]( https://en.wikipedia.org/wiki/Schnorr_signature )
 * Optimization: *native*: optimization 3,lto=true, *wasm*: optimization=s,lto=true,strip=true

#### Codegen-units 8

|   Runetime	|   Release	|   performance	|
|---		    |---		|---		    |
|   Native	    |   170 ms	|   	1	    |
|   Wasmtime	|   542 ms	|   	3.1	    |
|   Wasm3	    |   4.9 s	|   	28	    |

#### Codegen-units 1

|   Runetime	|   Release	|   performance	|
|---		    |---		|---		    |
|   Native	    |   167 ms	|   	1	    |
|   Wasmtime	|   470 ms	|   	2.7	    |
|   Wasm3	    |   4.9 s	|   	29	    |

### [ECDSA] 
 * 1024 messages, signing method: [ECDSA]( https://en.wikipedia.org/wiki/Elliptic_Curve_Digital_Signature_Algorithm )
 * Native platform: x86_64, Intel(R) Core(TM) i7-4771 CPU @ 8 cores @ 3.50GHz 
 * Optimization: *native*: optimization 3,lto=true, *wasm*: optimization=s,lto=true,strip=true

#### Codegen-units 8

|   Runetime	|   Release	|   performance	|
|---		    |---		|---		    |
|   Native	    |   189 ms	|   	1	    |
|   Wasmtime	|   720 ms	|   	3.7	    |
|   Wasm3	    |   5.1 s	|   	26	    |

#### Codegen-units 1

|   Runetime	|   Release	|   performance	|
|---		    |---		|---		    |
|   Native	    |   178 ms	|   	1	    |
|   Wasmtime	|   612 ms	|   	3.4	    |
|   Wasm3	    |   5.0 s	|   	28	    |

## Hardware Platform
* x86_64, Intel(R) Core(TM) i7-4771K CPU @ 8 cores @ 3.50GHz 
* Linux@6.2.1 x86_64

### [SCHNORR] 
 * 1024 messages, signing method: [Schnorr]( https://en.wikipedia.org/wiki/Schnorr_signature )
 * Optimization: *native*: optimization 3,lto=true, *wasm*: optimization=s,lto=true,strip=true

#### Codegen-units 8

|   Runetime	|   Release	|   performance	|
|---		    |---		|---		    |
|   Native	    |   130 ms	|   	1	    |
|   Wasmtime	|   395 ms	|   	3	    |
|   Wasm3	    |   3.91 s	|   	30	    |

#### Codegen-units 1

|   Runetime	|   Release	|   performance	|
|---		    |---		|---		    |
|   Native	    |   125 ms	|   	1	    |
|   Wasmtime	|   390 ms	|   	3.1	    |
|   Wasm3	    |   3.75 s	|   	30	    |

### [ECDSA]
 * 1024 messages, signing method: [ECDSA]( https://en.wikipedia.org/wiki/Elliptic_Curve_Digital_Signature_Algorithm )
 * Optimization: *native*: optimization 3,lto=true, *wasm*: optimization=s,lto=true,strip=true

#### Codegen-units 8

|   Runetime	|   Release	|   performance	|
|---		    |---		|---		    |
|   Native	    |   475 ms	|   	1	    |
|   Wasmtime	|   146 ms	|   	3.2	    |
|   Wasm3	    |   4.01 s	|   	27	    |

#### Codegen-units 1

|   Runetime	|   Release	|   performance	|
|---		    |---		|---		    |
|   Native	    |   145 ms	|   	1	    |
|   Wasmtime	|   450 ms	|   	3.1	    |
|   Wasm3	    |   3.87 s	|   	26	    |


## Module Swapping performance
+ **Random message** (SCHNORR or ECDSA) of count 1024, runetime module swapping, WASMTIME runtime
### Hardware Platform
* x86_64, Intel(R) Core(TM) i7-4771K CPU @ 8 cores @ 3.50GHz 
* Linux@6.2.1 x86_64
#### Result
+ **total message processing time = 31.2 s, i.e about  30 ms to process one message (include module swapping)**

## Single message performance

- Linux@6.2.1 x86_64, Schorr method: 

|   Runetime	|   Release	|   performance	|
|---		    |---		|---		    |
|   Native	    |   141 us	|   	1	    |
|   Wasmtime	|   422 us	|   	3	    |
|   Wasm3	    |   7.74 ms	|   	54	    |

## Conclusion

Being a "Web Assembly interpretier" [meta machine]( https://github.com/wasm3/wasm3/blob/main/docs/Interpreter.md#m3-massey-meta-machine ), wasm3 is expectindly demonstrate the worst performance,
being compared to native and Wasmtime platforms.
Wasmtime, which utilzes [AOT and JIT technologies]( https://github.com/bytecodealliance/wasmtime#features ), preforms much faster that wasm3,
but still, x3 slower, than compared to native x86_64 platform.

