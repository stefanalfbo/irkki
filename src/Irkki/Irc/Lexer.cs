namespace Irkki.Irc;

public enum TokenType
{
    Illegal,
    EOF,
    Prefix,
}

public record Token(TokenType Type, string Value);

public class Lexer
{
    private readonly string _input;
    private char _currentChar;
    private int _currentPosition;
    private int _readPosition;

    public Lexer(string input)
    {
        _input = input;
        _currentPosition = 0;
        _readPosition = 0;

        ReadChar();
    }

    private void ReadChar()
    {
        _currentChar = _readPosition >= _input.Length
            ? '\0'
            : _input[_readPosition];
 
        _currentPosition = _readPosition;
        _readPosition++;
    }
    
    private string ReadString()
    {
        var start = _currentPosition + 1;

        while (_currentChar != '\0' && _currentChar != ' ')
        {
            ReadChar();
        }

        return _input[start.._currentPosition];
    }

    public Token NextToken()
    {
        Token token;

        switch (_currentChar)
        {
            case ':':
                var value = ReadString();
                token = new Token(TokenType.Prefix, value);
                break;
            case '\0':
                return new Token(TokenType.EOF, string.Empty);

            default:
                return new Token(TokenType.Illegal, _currentChar.ToString());
        }

        ReadChar();

        return token;
    }
}