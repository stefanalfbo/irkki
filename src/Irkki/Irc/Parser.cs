namespace Irkki.Irc;

public class ParseException(string message) : Exception(message) { }

public class Parser
{
    private readonly Lexer _lexer;

    public Parser(Lexer lexer)
    {
        _lexer = lexer;
    }

    public Message ParseMessage()
    {
        var token = _lexer.NextToken();

        // Prefix handling
        string? prefix = PrefixHandler(token);
        if (prefix != null)
        {
            token = _lexer.NextToken();
        }

        var command = CommandHandler(token);

        return new Message(prefix, command, Array.Empty<string>());
    }

    private string? PrefixHandler(Token token)
    {
        string? prefix = null;
        if (token.Type == TokenType.Colon)
        {
            prefix = _lexer.NextToken().Value;
            if (_lexer.NextToken().Type != TokenType.Space)
            {
                throw new ParseException("Expected space after prefix.");
            }
        }

        return prefix;
    }

    private string CommandHandler(Token token)
    {
        if (token.Type != TokenType.Word)
        {
            throw new ParseException("Expected command token.");
        }

        if (!IsOnlyBasedOnLetters(token.Value))
        {
            throw new ParseException("Command must consist of letters only.");
        }

        return token.Value;
    }
    
    private bool IsOnlyBasedOnLetters(string value)
    {
        foreach (var c in value)
        {
            if (!char.IsAsciiLetter(c))
            {
                return false;
            }
        }
        return true;
    }
}