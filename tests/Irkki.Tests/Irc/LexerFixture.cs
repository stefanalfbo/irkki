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
}
