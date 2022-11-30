# CPU to CSV
Simple utility to dump the CPU usage in CSV format

### Requirements
Existence of */proc/stat* [only Linux OS]

Read permission to */proc/stat*


### Compiling
```bash
cpu_csv_dumper$ cargo build --release
```
Binary will be located under
*./target/release/cpu_csv_dumper*

### Basic usage
```bash
cpu_csv_dumper$ cpu_csv_dumper
cpu, cpu0, cpu1, cpu2, cpu3, cpu4, cpu5, cpu6, cpu7
5.8, 10.3, 6.7, 0.0, 6.5, 3.4, 6.7, 9.7, 6.7
2.9, 3.3, 3.2, 6.5, 3.3, 0.0, 3.3, 0.0, 0.0
4.2, 9.7, 0.0, 0.0, 0.0, 3.3, 6.5, 10.0, 6.5
4.6, 6.7, 0.0, 3.2, 9.7, 0.0, 3.6, 6.5, 0.0
2.9, 3.2, 3.2, 6.5, 3.3, 0.0, 3.2, 0.0, 3.2
...
```
### Other options
|  Command |  Description |
|--|--|
|  -t, --time \<TIME\> |  Milliseconds between each dump |
|-h, --help|   Print help information  |
| -V, --version|  Print version information|



