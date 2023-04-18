# Project RUST and WASM performance and runtime-reloadable modules

## Overview

This is a draft project which main purpose is to provide an experience 
working with various aspects of Rust language and WASM technology ecosystem and tools.
Finall goal of this project is to evaluate performance of code being executed on native 
platform and inside WASM runtimes(wasm3,wamtime) and demonstate possibility of runtime wasm module switching during runetime.
During project work I tried to utilize the most popular and usefull crates that can be applied to 
this work (tokio,tonic,redis etc..), also, investigating WASM runtimes and toolchain.

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

## Usage

### Requirements
    * rustc 1.70.0-nightly
    * docker & docker-compose (or redis installed)
    * protobuf
    * wasm32-unknown-unknown rustup target installed
### Usage
    1. `` docker-compose -f docker-compose-redis.yml up `` in redis-compose directory.
    2. Compile WASM module `` cargo build -p module4-verify --target wasm32-unknown-unknown --release ``
    3. Compile WASM module `` cargo build -p module6-verify --target wasm32-unknown-unknown --release ``
    4. Run `` cargo run -p server --release ``
    5. Run `` cargo run -p main-app --release -- --runner 0 `` to run example with module runtime replace

## Project structure
![](https://github.com/Kerosin3/RUST-WASM-RPC-PROJECT/blob/main/docs/shema.jpg)

## Results

### [SCHNOOR] Platform: linux@Windows-WSL x86_64
 * 1024 messages, signing method: [Schnorr]( https://en.wikipedia.org/wiki/Schnorr_signature )
 * Native platform: x86_64, Intel(R) Core(TM) i7-4770K CPU @ 8 cores @ 3.50GHz 
 * Optimization: *native*: optimization 3,lto=true, *wasm*: optimization=s,lto=true,strip=true

#### Codegen-units 8

|   Runetime	|   Release	|   performance	|
|---		    |---		|---		    |
|   Native	    |   234 ms	|   	1	    |
|   Wasmtime	|   630 ms	|   	2.7	    |
|   Wasm3	    |   5.51 s	|   	23	    |

#### Codegen-units 1

|   Runetime	|   Release	|   performance	|
|---		    |---		|---		    |
|   Native	    |   208 ms	|   	1	    |
|   Wasmtime	|   603 ms	|   	2.9	    |
|   Wasm3	    |   5.5 s	|   	26	    |

### [ECDSA] Platform: linux@Windows-WSL x86_64
 * 1024 messages, signing method: [ECDSA]( https://en.wikipedia.org/wiki/Elliptic_Curve_Digital_Signature_Algorithm )
 * Native platform: x86_64, Intel(R) Core(TM) i7-4771 CPU @ 8 cores @ 3.50GHz 
 * Optimization: *native*: optimization 3,lto=true, *wasm*: optimization=s,lto=true,strip=true

#### Codegen-units 8

|   Runetime	|   Release	|   performance	|
|---		    |---		|---		    |
|   Native	    |   208 ms	|   	1	    |
|   Wasmtime	|   615 ms	|   	2.9	    |
|   Wasm3	    |   5.1 s	|   	24	    |

#### Codegen-units 1

|   Runetime	|   Release	|   performance	|
|---		    |---		|---		    |
|   Native	    |   251 ms	|   	1	    |
|   Wasmtime	|   753 ms	|   	3	    |
|   Wasm3	    |   6.2 s	|   	24	    |
