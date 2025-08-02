namespace Irkki.Irc;

public enum TokenType
{
    Illegal,
    EOF,
    CrLf,
    Colon,
    Space,
    Word
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
        var start = _currentPosition;
        var nextChar = PeekChar();

        while (nextChar != '\r' && nextChar != ' ' && nextChar != '\0')
        {
            ReadChar();
            nextChar = PeekChar();
        }

        return _input[start..(_currentPosition+1)];
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
                if (_currentPosition == 0 && _readPosition == 1)
                {
                    // Special case for leading colon in prefix
                    token = new Token(TokenType.Colon, ":");
                }
                else
                {
                    token = new Token(TokenType.Word, ReadString());
                }
                break;
            case ' ':
                token = new Token(TokenType.Space, _currentChar.ToString());
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
                token = new Token(TokenType.EOF, string.Empty);
                break;
            default:
                token = new Token(TokenType.Word, ReadString());
                break;
        }

        ReadChar();

        return token;
    }
}