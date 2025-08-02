namespace Irkki.Irc;

public enum TokenType
{
    Illegal,
    CrLf,
    Colon,
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

    private char PeekChar()
    {
        return _readPosition >= _input.Length ? '\0' : _input[_readPosition];
    }

    public Token NextToken()
    {
        Token token;

        switch (_currentChar)
        {
            case ':':
                token = new Token(TokenType.Colon, _currentChar.ToString());
                break;
            case '\r':
                if (PeekChar() == '\n')
                {
                    ReadChar();
                    token = new Token(TokenType.CrLf, "\r\n");
                }
                else
                {
                    token = new Token(TokenType.Illegal, _currentChar.ToString());
                }
                break;
            case '\0':
                return new Token(TokenType.Illegal, string.Empty);

            default:
                return new Token(TokenType.Illegal, _currentChar.ToString());
        }

        ReadChar();

        return token;
    }
}