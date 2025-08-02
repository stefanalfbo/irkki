using Irkki.Irc;

namespace Irkki.Tests.Irc;

public class LexerFixture
{
    [Fact]
    public void TestIllegalToken()
    {
        // Arrange
        var message = "\r";
        var lexer = new Lexer(message);

        // Act
        var token = lexer.NextToken();

        // Assert
        Assert.Equal(TokenType.Illegal, token.Type);
        Assert.Equal("\r", token.Value);
    }

    [Fact]
    public void TestEOFToken()
    {
        // Arrange
        var message = "\0";
        var lexer = new Lexer(message);

        // Act
        var token = lexer.NextToken();

        // Assert
        Assert.Equal(TokenType.EOF, token.Type);
        Assert.Equal("", token.Value);
    }

    [Fact]
    public void TestCrLfToken()
    {
        // Arrange
        var message = "\r\n";
        var lexer = new Lexer(message);

        // Act
        var token = lexer.NextToken();

        // Assert
        Assert.Equal(TokenType.CrLf, token.Type);
        Assert.Equal("\r\n", token.Value);
    }

    [Fact]
    public void TestColonToken()
    {
        // Arrange
        var message = ":copper.libera.chat NOTICE * :*** Checking Ident\r\n";
        var lexer = new Lexer(message);

        // Act
        var token = lexer.NextToken();

        // Assert
        Assert.Equal(TokenType.Colon, token.Type);
        Assert.Equal(":", token.Value);
    }

    [Fact]
    public void TestSpaceToken()
    {
        // Arrange
        var message = " ";
        var lexer = new Lexer(message);

        // Act
        var token = lexer.NextToken();

        // Assert
        Assert.Equal(TokenType.Space, token.Type);
        Assert.Equal(" ", token.Value);
    }

    [Fact]
    public void TestWordToken()
    {
        // Arrange
        var message = "Hello";
        var lexer = new Lexer(message);

        // Act
        var token = lexer.NextToken();

        // Assert
        Assert.Equal(TokenType.Word, token.Type);
        Assert.Equal("Hello", token.Value);
    }

    [Fact]
    public void TestMessage()
    {
        // Arrange
        var message = ":copper.libera.chat NOTICE\r\n";
        var lexer = new Lexer(message);

        // Act
        var tokens = new List<Token>();
        Token token;
        do
        {
            token = lexer.NextToken();
            tokens.Add(token);
        } while (token.Type != TokenType.EOF);

        // Assert
        Assert.Equal(6, tokens.Count);

        Assert.Equal(TokenType.Colon, tokens[0].Type);
        Assert.Equal(":", tokens[0].Value);

        Assert.Equal(TokenType.Word, tokens[1].Type);
        Assert.Equal("copper.libera.chat", tokens[1].Value);

        Assert.Equal(TokenType.Space, tokens[2].Type);
        Assert.Equal(" ", tokens[2].Value);

        Assert.Equal(TokenType.Word, tokens[3].Type);
        Assert.Equal("NOTICE", tokens[3].Value);

        Assert.Equal(TokenType.CrLf, tokens[4].Type);
        Assert.Equal("\r\n", tokens[4].Value);

        Assert.Equal(TokenType.EOF, tokens[5].Type);
        Assert.Equal("", tokens[5].Value);
    }
}
