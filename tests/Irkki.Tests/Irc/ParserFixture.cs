using Irkki.Irc;

namespace Irkki.Tests.Irc;

public class ParserFixture
{
    [Fact]
    public void TestExtractingPrefixFromMessage()
    {
        // Arrange
        var message = ":copper.libera.chat NOTICE * :*** Checking Ident\r\n";
        var lexer = new Lexer(message);
        var parser = new Parser(lexer);

        // Act
        var parsedMessage = parser.ParseMessage();

        // Assert
        Assert.Equal("copper.libera.chat", parsedMessage.Prefix);
    }

    [Fact]
    public void TestMessageWithoutPrefix()
    {
        // Arrange
        var message = "NOTICE * :*** Checking Ident\r\n";
        var lexer = new Lexer(message);
        var parser = new Parser(lexer);

        // Act
        var parsedMessage = parser.ParseMessage();

        // Assert
        Assert.Null(parsedMessage.Prefix);
    }

    [Fact]
    public void TestExtractingNoticeCommand()
    {
        // Arrange
        var message = ":copper.libera.chat NOTICE * :*** Checking Ident\r\n";
        var lexer = new Lexer(message);
        var parser = new Parser(lexer);

        // Act
        var parsedMessage = parser.ParseMessage();

        // Assert
        Assert.Equal("NOTICE", parsedMessage.Command);
    }
}