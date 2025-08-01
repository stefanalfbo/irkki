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
        var prefix = lexer.NextToken();

        // Assert
        Assert.Equal(TokenType.Prefix, prefix.Type);
        Assert.Equal("copper.libera.chat", prefix.Value);
    }
}
