using Irkki.Irc;

namespace Irkki.Tests.Irc;

public class ParserFixture
{
    [Fact]
    public void TestExtractingPrefixFromMessage()
    {
        // Arrange
        var message = ":copper.libera.chat NOTICE * :*** Checking Ident\r\n";
        var parser = new Parser(message);

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
        var parser = new Parser(message);

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
        var parser = new Parser(message);

        // Act
        var parsedMessage = parser.ParseMessage();

        // Assert
        Assert.Equal("NOTICE", parsedMessage.Command);
    }

    [Fact]
    public void TestInvalidCommandThrowsException()
    {
        // Arrange
        var message = ":copper.libera.chat N0T1C3 * :*** Checking Ident\r\n";
        var parser = new Parser(message);

        // Act & Assert
        Assert.Throws<ParseException>(() => parser.ParseMessage());
    }

    [Fact]
    public void TestNumericCommand()
    {
        // Arrange
        var message = ":copper.libera.chat 001 copper :Welcome to the IRC server\r\n";
        var parser = new Parser(message);

        // Act
        var parsedMessage = parser.ParseMessage();

        // Assert
        Assert.Equal("001", parsedMessage.Command);
    }
}