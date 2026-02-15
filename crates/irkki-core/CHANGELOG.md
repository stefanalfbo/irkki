# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1](https://github.com/stefanalfbo/irkki/compare/v0.1.0...v0.1.1) - 2026-02-15

### Other

- Implement /whois command in IRCClient and enhance message submission logic
- Add IRCClient to App struct and implement message sending functionality to the channel.
- Hanlde message of the day commands, 372, 375 and 376
- Move handling of command numbers into the irc client and use the IRCEvent type instead
- Refactor how parsed messages are processed in the irc client
- Refactor the code and push the parser down to the irc client from the app code
- Make a first attempt to set up a test stub irc server for testing
- Add a test for illegal token in the lexer
- Add error logging to the parser
- Add at least one test to the irc_client module
