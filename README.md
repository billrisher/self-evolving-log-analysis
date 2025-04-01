# Self-Evolving Log Analysis Tool

## Build Instructions

1. Clone this repository
    - `git clone https://github.com/billrisher/self-evolving-log-analysis.git`
2. Change into the directory
    - `cd self-evolving-log-analysis`
3. Run `cargo build --release`
4. Run the program, located at `./target/release/log_analysis`

## Usage

`./log_emitter | ./target/release/log_analysis`

### Command Line Arguments

- `-debug` : Enable debug mode

## Overview of Requirements

This project is a primitive log analysis tool that parses logs, runs analysis on their contents, and prints the results.

It is designed to be robust, handling large bursts of logs with ease.

It does NOT yet have the ability to dynamically adjust the analysis window, adjust the queue buffer, or adjust the
analysis.

### Known Limitations

As it stands, the tool is not capable of handling much more throughput than about 1,000 logs per second.
This is partially due to improper mutex implementations, which could be sped up through better granularity on the
locks (e.g. using a read-write lock so that we can still output data every second).