using System;
using System.Net.Http;
using System.Text.Json;
using System.Threading.Tasks;
using BenchmarkDotNet.Attributes;
using BenchmarkDotNet.Running;

namespace Benchmark
{
    public class Note
    {
        public int id { get; set; }
        public string text { get; set; }
    }

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
            private const int Count = 300;
            private static readonly HttpClient Client = new HttpClient();

            private static async Task Step(string url)
            {
                var post = await Client
                    .PostAsync(url, new StringContent(Guid.NewGuid().ToString()))
                    .ConfigureAwait(false);

                var posted = await JsonSerializer
                    .DeserializeAsync<Note>(
                        await post.Content
                            .ReadAsStreamAsync()
                            .ConfigureAwait(false))
                    .ConfigureAwait(false);

                var get = await Client.GetStreamAsync($"{url}/{posted.id}").ConfigureAwait(false);

                var note = await JsonSerializer
                    .DeserializeAsync<Note>(get)
                    .ConfigureAwait(false);

                if (note.text != posted.text) throw new InvalidOperationException();
            }

            [Benchmark]
            public void R2D2()
            {
                for (var i = 0; i < Count; i++) Step("http://localhost:8080").GetAwaiter().GetResult();
            }

            [Benchmark]
            public void BB8()
            {
                for (var i = 0; i < Count; i++) Step("http://localhost:9080").GetAwaiter().GetResult();
            }

            [Benchmark]
            public void Asp()
            {
                for (var i = 0; i < Count; i++) Step("http://localhost:5000").GetAwaiter().GetResult();
            }
        }
    }
}