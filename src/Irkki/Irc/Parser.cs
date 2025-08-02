namespace Irkki.Irc;

public class Parser
{
    private readonly Lexer _lexer;

    public Parser(Lexer lexer)
    {
        _lexer = lexer;
    }

    public Message ParseMessage()
    {
        string? prefix = null;
        if (_lexer.NextToken().Type == TokenType.Colon)
        {
            prefix = _lexer.NextToken().Value;
        }

        return new Message(prefix, "not implemented yet", Array.Empty<string>());
    }
}