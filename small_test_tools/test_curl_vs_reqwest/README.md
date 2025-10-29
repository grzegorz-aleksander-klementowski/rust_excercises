# curl vs reqwest benchmark

This is a tiny Rust playground that fires 100 sequential GET requests against `http://example.com` using three different approaches: the `curl` command-line tool (spawned from Rust), the `curl` crate (libcurl bindings), and `reqwest`'s blocking client. The goal is to compare their wall-clock behaviour under identical conditions.

## Findings

Benchmarks were captured with `time` while running each binary once:

| implementation | real time | user time | sys time |
| --- | --- | --- | --- |
| curl CLI (`src/bin/test_curl.rs`) | 57.273s | 0.672s | 0.637s |
| libcurl via crate (`src/bin/test_libcurl.rs`) | 13.108s | 0.015s | 0.027s |
| reqwest blocking (`src/bin/program_reqwest.rs`) | 34.158s | 2.069s | 0.131s |

The libcurl bindings finish fastest by reusing a single client handle. Spawning the external `curl` process for every request is the slowest path, while `reqwest` lands in the middle thanks to native Rust implementation overhead but without the cost of process creation.
