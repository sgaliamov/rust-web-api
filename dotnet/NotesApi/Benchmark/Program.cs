using System;
using System.Linq;
using System.Net.Http;
using System.Threading;
using System.Threading.Tasks;
using BenchmarkDotNet.Attributes;
using BenchmarkDotNet.Running;
using Dapper;
using Npgsql;

namespace Benchmark
{
    public sealed class Program
    {
        public static void Main()
        {
            BenchmarkRunner.Run<Benchmarks>();
        }

        [MedianColumn]
        [RankColumn]
        public class Benchmarks
        {
            private const string ConnectionString =
                "User ID=postgres;Password=postgres;Host=localhost;Port=5432;Database=postgres;Pooling=true;";

            private const int Count = 100;
            private static readonly HttpClient Client = new HttpClient();
            private static readonly SemaphoreSlim Semaphore = new SemaphoreSlim(4);

            public static async Task Step(string url)
            {
                await Semaphore.WaitAsync(CancellationToken.None);

                var post = await Client
                    .PostAsync(url, new StringContent(Guid.NewGuid().ToString()))
                    .ConfigureAwait(false);

                var posted = await post.Content.ReadAsStringAsync().ConfigureAwait(false);
                var id = posted.Split("\"id\":").Last().Split(",").First();
                if (!string.IsNullOrWhiteSpace(id))
                {
                    var note = await Client.GetStringAsync($"{url}/{id}").ConfigureAwait(false);
                    if (note != posted)
                    {
                        throw new InvalidOperationException($"get: {note}\npost: {posted}");
                    }
                }

                Semaphore.Release();
            }

            [Benchmark]
            public void R2D2_Parallel() => RunInParallel(_ => Step("http://localhost:8080"));

            [Benchmark]
            public void BB8_Parallel() => RunInParallel(_ => Step("http://localhost:9080"));

            [Benchmark]
            public void Asp_Parallel() => RunInParallel(_ => Step("http://localhost:5000"));

            [Benchmark]
            public void R2D2_Sequence()
            {
                RunInSequence("http://localhost:8080");
            }

            [Benchmark]
            public void BB8_Sequence()
            {
                RunInSequence("http://localhost:9080");
            }

            [Benchmark]
            public void Asp_Sequence()
            {
                RunInSequence("http://localhost:5000");
            }

            public static void RunInParallel(Func<int, Task> selector)
            {
                var tasks = Enumerable.Range(0, Count).AsParallel().Select(selector).ToArray();
                Task.WaitAll(tasks);
            }

            [GlobalSetup]
            public void GlobalSetup()
            {
                using var connection = new NpgsqlConnection(ConnectionString);

                connection.Execute("drop table if exists notes");
                Thread.Sleep(1000);
                connection.Execute(@"
                    create table if not exists notes (
                        id serial,
                            text varchar not null,
                        timestamp timestamptz default now() not null)");
                Thread.Sleep(1000);
            }

            private static void RunInSequence(string url)
            {
                for (var i = 0; i < Count; i++)
                {
                    Step(url).GetAwaiter().GetResult();
                }
            }
        }
    }
}