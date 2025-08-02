using Irkki.Irc;

namespace Irkki.Tests.Irc;

public class LexerFixture
{
    [Fact]
    public void TestIllegalToken()
    {
        // Arrange
        var message = "\0";
        var lexer = new Lexer(message);

        // Act
        var token = lexer.NextToken();

        // Assert
        Assert.Equal(TokenType.Illegal, token.Type);
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
    public void TestMessage()
    {
        // Arrange
        var message = ": \r\n";
        var lexer = new Lexer(message);

        // Act
        var tokens = new List<Token>();
        Token token;
        do
        {
            token = lexer.NextToken();
            tokens.Add(token);
        } while (token.Type != TokenType.CrLf);

        // Assert
        Assert.Equal(3, tokens.Count);

        Assert.Equal(TokenType.Colon, tokens[0].Type);
        Assert.Equal(":", tokens[0].Value);

        Assert.Equal(TokenType.Space, tokens[1].Type);
        Assert.Equal(" ", tokens[1].Value);

        Assert.Equal(TokenType.CrLf, tokens[2].Type);
        Assert.Equal("\r\n", tokens[2].Value);
    }
}
