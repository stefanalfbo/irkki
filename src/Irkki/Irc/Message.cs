namespace Irkki.Irc;

public record Message(string? Prefix, string Command, string[] Parameters);
