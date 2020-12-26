using System;
using System.Globalization;
using System.IO;
using System.Text;
using System.Threading.Tasks;
using Dapper;
using Microsoft.AspNetCore.Mvc;
using Npgsql;

namespace NotesApi.Controllers
{
    [Route("/")]
    [ApiController]
    public class NotesController : ControllerBase
    {
        private const string ConnectionString =
            "User ID=postgres;Password=postgres;Host=localhost;Port=5432;Database=postgres;Pooling=true;";

        [HttpPost]
        public async Task<IActionResult> Add()
        {
            using var reader = new StreamReader(Request.Body, Encoding.UTF8);

            var text = await reader
                .ReadToEndAsync()
                .ConfigureAwait(false);

            await using var connection = new NpgsqlConnection(ConnectionString);

            var inserted = await connection
                .QueryFirstOrDefaultAsync<NoteId>(@"
                    insert into notes (text)
                        values (@text)
                        returning id, timestamp", new { text })
                .ConfigureAwait(false);

            return Ok(new Note(inserted.Id, text, inserted.Timestamp));
        }

        [HttpGet("{id}")]
        public async Task<Note> Get(int id)
        {
            await using var connection = new NpgsqlConnection(ConnectionString);

            return await connection
                .QueryFirstOrDefaultAsync<Note>(@"
                    select id, text, timestamp
                        from notes
                        where id = @id", new { id })
                .ConfigureAwait(false);
        }

        [HttpGet("date")]
        public string GetDate() => DateTime.UtcNow.ToString(CultureInfo.InvariantCulture);
    }

    public record NoteId(int Id, DateTime Timestamp);

    public record Note(int Id, string Text, DateTime Timestamp) : NoteId(Id, Timestamp);
}