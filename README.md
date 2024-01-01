# Blaze Turbo 

![](asset/logo.png)

Blaze Turbo(风火轮) is a high-performance key-value database engine that serves as a multi-threaded, persistent key/value store server and client. It utilizes synchronous networking over a custom protocol.

## Features

- [x] Handle and report errors robustly
- [x] Use `serde` for serialization
- [x] Write data to disk as a log using standard file APIs
- [x] Read the state of the key/value store from disk
- [x] Map in-memory key-indexes to on-disk values
- [x] Periodically compact the log to remove stale data
- [x] Use channels for cross-thread communication
- [x] Share data structures with locks
- [x] Perform read operations without locks
- [x] Benchmark single-threaded vs multithreaded

**Topics**: log-structured file I/O, bitcask, the failure crate, Read / Write traits, the serde crate, thread pools, channels, locks, lock-free data structures, atomics, parameterized benchmarking.

## Installation

To install Blaze Turbo, you can clone the repository from GitHub:

```sh
git clone https://github.com/roll-king/blaze-turbo.git
```
## Usage

Build the project using Cargo:

```sh
cd blaze-turbo
cargo build --release
```

Start the server:
```sh
./target/release/blaze-server
```
Use the client to interact with the server:
```sh
./target/release/blaze-client
```
## Contributing

Contributions to Blaze Turbo are welcome! If you find any bugs or have suggestions for new features, please open an issue on the GitHub repository. You can also submit pull requests with your proposed changes.

## Acknowledgements

Blaze Turbo is built upon the valuable lessons and materials provided by the Talent Plan training program. The [Talent Plan](https://github.com/OneSizeFitsQuorum/talent-plan) is an open source training program initiated by PingCAP. We would like to thank PingCAP for their initiative in creating this program.

![](https://github.com/OneSizeFitsQuorum/talent-plan/raw/master/media/talent-plan-logo.png)

We used the GPT4, Copilot and DALLE4 in the process of designing the logo, writing documentation and code.

## License

Blaze Turbo is licensed under the [GPLv3](./LICENSE) License.