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

| Method |    Mean |    Error |   StdDev |  Median | Rank |
|------- |--------:|---------:|---------:|--------:|-----:|
|   Rust | 8.525 s | 0.1662 s | 0.1848 s | 8.468 s |    1 |
|    Asp | 8.956 s | 0.1578 s | 0.1318 s | 8.931 s |    2 |
