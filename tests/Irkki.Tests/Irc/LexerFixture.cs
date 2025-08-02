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
        var message = ":copper.libera.chat NOTICE * :*** Checking Ident\r\n";
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
        Assert.Equal(14, tokens.Count);

        Assert.Equal(TokenType.Colon, tokens[0].Type);
        Assert.Equal(":", tokens[0].Value);

        Assert.Equal(TokenType.Word, tokens[1].Type);
        Assert.Equal("copper.libera.chat", tokens[1].Value);

        Assert.Equal(TokenType.Space, tokens[2].Type);
        Assert.Equal(" ", tokens[2].Value);

        Assert.Equal(TokenType.Word, tokens[3].Type);
        Assert.Equal("NOTICE", tokens[3].Value);

        Assert.Equal(TokenType.Space, tokens[4].Type);
        Assert.Equal(" ", tokens[4].Value);

        Assert.Equal(TokenType.Word, tokens[5].Type);
        Assert.Equal("*", tokens[5].Value);

        Assert.Equal(TokenType.Space, tokens[6].Type);
        Assert.Equal(" ", tokens[6].Value);

        Assert.Equal(TokenType.Word, tokens[7].Type);
        Assert.Equal(":***", tokens[7].Value);

        Assert.Equal(TokenType.Space, tokens[8].Type);
        Assert.Equal(" ", tokens[8].Value);

        Assert.Equal(TokenType.Word, tokens[9].Type);
        Assert.Equal("Checking", tokens[9].Value);

        Assert.Equal(TokenType.Space, tokens[10].Type);
        Assert.Equal(" ", tokens[10].Value);

        Assert.Equal(TokenType.Word, tokens[11].Type);
        Assert.Equal("Ident", tokens[11].Value);

        Assert.Equal(TokenType.CrLf, tokens[12].Type);
        Assert.Equal("\r\n", tokens[12].Value);

        Assert.Equal(TokenType.EOF, tokens[13].Type);
        Assert.Equal("", tokens[13].Value);
    }
}
