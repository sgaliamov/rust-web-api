# rust-web-api

Sample web API server on Rust.

## Does

1. Add a record to SQL DB.
2. Get a record from SQL DB.

## How to run

``` ps1
.\scripts\run.ps1
```

### Results

``` ini
BenchmarkDotNet=v0.12.1, OS=Windows 10.0.19042
Intel Core i7-8750H CPU 2.20GHz (Coffee Lake), 1 CPU, 12 logical and 6 physical cores
.NET Core SDK=5.0.101
  [Host]     : .NET Core 5.0.1 (CoreCLR 5.0.120.57516, CoreFX 5.0.120.57516), X64 RyuJIT
  DefaultJob : .NET Core 5.0.1 (CoreCLR 5.0.120.57516, CoreFX 5.0.120.57516), X64 RyuJIT
```

| Method        |     Mean |    Error |   StdDev |   Median |
| ------------- | -------: | -------: | -------: | -------: |
| Asp_Parallel  | 223.9 ms |  9.87 ms | 29.11 ms | 226.0 ms |
| R2D2_Parallel | 241.9 ms |  4.80 ms |  4.25 ms | 242.1 ms |
| BB8_Parallel  | 283.3 ms |  5.64 ms | 10.31 ms | 284.5 ms |
| R2D2_Sequence | 904.4 ms | 17.89 ms | 44.89 ms | 910.2 ms |
| BB8_Sequence  | 966.2 ms | 18.56 ms | 18.22 ms | 964.7 ms |
| Asp_Sequence  | 511.6 ms | 20.43 ms | 60.23 ms | 519.0 ms |
