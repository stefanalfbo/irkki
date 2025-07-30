using System.Net.Sockets;

namespace Irkki.Irc;

public class IrcClient
{
    public string Nickname { get; private set; } = "anonguest4523";
    private string _server = "localhost";
    private int _port = 6667;
    private string _channel = "#testchannel";

    private TcpClient? _client;
    private StreamReader? _reader;
    private StreamWriter? _writer;

    private IrcClient() { throw new InvalidOperationException("Use Connect method to create an instance."); }

    private IrcClient(string nickname, string server, int port)
    {
        _server = server;
        _port = port;
        Nickname = nickname;
    }

    private async Task Connect()
    {
        _client = new TcpClient();
        await _client.ConnectAsync(_server, _port);

        var stream = _client.GetStream();
        _reader = new StreamReader(stream);
        _writer = new StreamWriter(stream) { NewLine = "\r\n", AutoFlush = true };

        await _writer.WriteLineAsync($"NICK {Nickname}");
        await _writer.WriteLineAsync($"USER {Nickname} 0 * :{Nickname}");
        await _writer.WriteLineAsync($"JOIN {_channel}");
    }

    static public async Task<IrcClient> Connect(string nickname, string server, int port)
    {
        var irc = new IrcClient(nickname, server, port);
        await irc.Connect();
        return irc;
    }

    public async Task Listen(Func<string, Task> messageHandler)
    {
        if (_client == null || _reader == null || _writer == null)
            throw new InvalidOperationException("Client is not connected.");
        
        while (true)
        {
            string? line;
            try
            {
                line = await _reader.ReadLineAsync();
                if (line == null)
                    break;

                if (line.StartsWith("PING"))
                {
                    string response = line.Replace("PING", "PONG");
                    await _writer.WriteLineAsync(response);
                    continue;
                }

                await messageHandler(line);
            }
            catch
            {
                break;
            }
        }
    }

    public async Task Quit()
    {
        if (_client == null || _writer == null)
            return;
        
        await _writer.WriteLineAsync($"PART {_channel} :Goodbye!");
        await _writer.WriteLineAsync("QUIT :Client closed");
    }
}