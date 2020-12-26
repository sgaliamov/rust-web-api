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
        public class Benchmarks
        {
            private const string ConnectionString =
                "User ID=postgres;Password=postgres;Host=localhost;Port=5432;Database=postgres;Pooling=true;";

            private const int Count = 500;
            private static readonly HttpClient Client = new HttpClient();
            private static readonly SemaphoreSlim Semaphore = new SemaphoreSlim(10);

            [Params(8080, 9080, 5000)]
            public int Port { get; set; }

            [Params(true, false)]
            public bool Parallel { get; set; }

            [Params("", "/date")]
            public string Path { get; set; }

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
            public void Benchmark()
            {
                var url = $"http://localhost:{Port}{Path}";

                if (Parallel)
                {
                    RunInParallel(_ => Step(url));
                }
                else
                {
                    RunInSequence(url);
                }
            }

            private static void RunInParallel(Func<int, Task> selector)
            {
                var tasks = Enumerable.Range(0, Count).AsParallel().Select(selector).ToArray();
                Task.WaitAll(tasks);
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