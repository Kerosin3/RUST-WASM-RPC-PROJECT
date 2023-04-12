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
* WASM
* WASM runtimes and tools (wasm3, wasmtime, etc..)
* WASM RPC (waPC)
* [Cryptography] ( https://crates.io/crates/k256 )
* Rust Unsafe
* gRPC & protobuf (via tonic & prost)
* NoSQL (Redis)
* Tokio
* Shared memory
* Serialization/Deserialzation & bin/hex code streams

## Usage

## Test stand


## Results

### Tesbench setup
 * 1024 messages, signing method: [Schnorr] ( https://en.wikipedia.org/wiki/Schnorr_signature )
 * Native platform: x86_64, Intel(R) Core(TM) i7-4771 CPU @ 8 cores @ 3.50GHz 
 * Optimization: native: optimization 3,lto=true, wasm: optimization=s,lto=true,strip=true


|   Runetime	|   Release	|   performance	|   Debug	|   Performance	|
|---		|---		|---		|---		|---		|
|   Native	|   130 ms	|   	1	|   1.79 s	|   1		|
|   Wasmtime	|   400 ms	|   	3	|   4.23 s	|   2.36	|
|   Wasm3	|   3.8 s	|   	30	|   *	|   	|
