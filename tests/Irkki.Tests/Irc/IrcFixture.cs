using Irkki.Irc;

namespace Irkki.Tests.Irc;

public class IrcFixture
{
    [Fact]
    public void TestLexer()
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
