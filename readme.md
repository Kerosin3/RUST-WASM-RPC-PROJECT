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
* Rust UNSAFE
* gRPC & protobuf (via tonic & prost)
* NoSQL (Redis)
* Tokio
* Shared memory
* Serialization/Deserialzation & bin/hex code streams

## Usage

## Test stand


## Results
* elapsed time
|   Renetime	|   release|   	|   debug	|
|---	|---	|---	|---	|
|   Native	|   	|   	|   	|
|   Wasmtime	|   	|   	|   	|
|   Wasm3	|   	|   	|   	|
