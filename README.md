# rust-web-api

Sample web API server on Rust.

There are `r2d2` and `bb8` based implementations here, as well as `asp.net` for comparison.

## This does

1. Adds a note to SQL DB.
1. Returns a note from Postgres DB.
1. Returns current date.

## Results

``` ini
BenchmarkDotNet=v0.12.1, OS=Windows 10.0.19042
Intel Core i7-8750H CPU 2.20GHz (Coffee Lake), 1 CPU, 12 logical and 6 physical cores
.NET Core SDK=5.0.101
  [Host]     : .NET Core 5.0.1 (CoreCLR 5.0.120.57516, CoreFX 5.0.120.57516), X64 RyuJIT
  DefaultJob : .NET Core 5.0.1 (CoreCLR 5.0.120.57516, CoreFX 5.0.120.57516), X64 RyuJIT
```

### ASP.NET

| Method    | Parallel | Path       |         Mean |       Error |      StdDev |       Median |
| --------- | -------- | ---------- | -----------: | ----------: | ----------: | -----------: |
| Benchmark | True     | GET /date  |     9.764 ms |   0.1692 ms |   0.1500 ms |     9.713 ms |
| Benchmark | False    | GET /date  |    43.043 ms |   0.8405 ms |   0.7862 ms |    42.742 ms |
| Benchmark | True     | POST & GET |   589.727 ms |  40.3020 ms | 118.8312 ms |   596.048 ms |
| Benchmark | False    | POST & GET | 3,157.214 ms | 231.7176 ms | 675.9310 ms | 3,165.454 ms |

### BB8

| Method    | Parallel | Path       |         Mean |       Error |       StdDev |       Median |
| --------- | -------- | ---------- | -----------: | ----------: | -----------: | -----------: |
| Benchmark | True     | GET /date  |     9.054 ms |   0.2258 ms |    0.6478 ms |     8.990 ms |
| Benchmark | False    | GET /date  |    37.105 ms |   0.7373 ms |    1.6941 ms |    36.482 ms |
| Benchmark | True     | POST & GET |   888.076 ms |  48.6239 ms |   142.605 ms |   867.573 ms |
| Benchmark | False    | POST & GET | 5,031.185 ms | 368.6956 ms | 1,087.107 ms | 4,895.177 ms |

### R2D2

| Method    | Parallel | Path       |         Mean |       Error |      StdDev |       Median |
| --------- | -------- | ---------- | -----------: | ----------: | ----------: | -----------: |
| Benchmark | True     | GET /date  |     8.945 ms |   0.2274 ms |   0.6596 ms |     8.935 ms |
| Benchmark | False    | GET /date  |    39.317 ms |   0.7753 ms |   1.7969 ms |    38.967 ms |
| Benchmark | True     | POST & GET |   976.429 ms |  51.5924 ms | 152.1214 ms |   966.551 ms |
| Benchmark | False    | POST & GET | 5,042.993 ms | 281.0737 ms | 828.7520 ms | 5,045.977 ms |
