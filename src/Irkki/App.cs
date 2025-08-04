using Spectre.Console;
using Irkki.Irc;

namespace Irkki;

public class App
{
    static IrcClient? _irc;
    static List<string> _messages = new();
    static List<string> _users = new();
    static string _nickname = "anonguest4523";
    static string _server = "irc.eu.libera.chat";
    static int _port = 6667;

    public async Task<int> RunAsync()
    {
        var currentScreen = AppScreen.Start;

        while (currentScreen != AppScreen.Exit)
        {
            switch (currentScreen)
            {
                case AppScreen.Start:
                    currentScreen = ShowStartScreen();
                    break;
                case AppScreen.Connect:
                    currentScreen = ShowConnectScreen();
                    _irc = await IrcClient.Connect(_nickname, _server, _port);
                    _ = Task.Run(() => _irc.Listen(async message =>
                    {
                        _messages.Add(message);
                        await Task.CompletedTask;
                    }));
                    break;
                case AppScreen.Main:
                    currentScreen = ShowMainScreen();
                    break;
                case AppScreen.Exit:
                    break;
            }
        }

        AnsiConsole.Clear();

        return 0;
    }

    static AppScreen ShowStartScreen()
    {
        AnsiConsole.Clear();

        AnsiConsole.Write(
            new FigletText("i r k k i")
                .Centered()
                .Color(Color.Green));

        var choice = AnsiConsole.Prompt(
            new SelectionPrompt<string>()
                .Title("[green]Choose an option:[/]")
                .AddChoices("Start", "Exit")
                .HighlightStyle(Color.Green));

        return choice switch
        {
            "Start" => AppScreen.Connect,
            _ => AppScreen.Exit
        };
    }

    static AppScreen ShowConnectScreen()
    {
        AnsiConsole.Clear();

        AnsiConsole.Write(
            new FigletText("i r k k i")
                .Centered()
                .Color(Color.Green));

        _nickname = AnsiConsole.Prompt(
            new TextPrompt<string>("[green]Enter your nickname:[/]")
                .DefaultValue(_nickname));

        _server = AnsiConsole.Prompt(
            new TextPrompt<string>("[green]Enter server address:[/]")
                .DefaultValue(_server));

        _port = AnsiConsole.Prompt(
            new TextPrompt<int>("[green]Enter server port:[/]")
                .DefaultValue(_port));

        return AppScreen.Main;

    }

    static AppScreen ShowMainScreen()
    {
        var totalHeight = Console.WindowHeight;
        int layoutHeight = totalHeight - 2;
        string currentInput = "";
        DateTime lastUpdate = DateTime.MinValue;

        while (true)
        {
            if (_messages.Count > 0 && DateTime.Now - lastUpdate > TimeSpan.FromMilliseconds(100))
            {
                AnsiConsole.Clear();

                var layout = CreateWindow();
                layout["Left"].Update(CreateChatPanel(_messages));
                layout["Right"].Update(CreateUserListPanel(_users));
                layout["Bottom"].Update(CreateInputPanel(currentInput));

                AnsiConsole.Write(layout);

                Console.SetCursorPosition(4 + currentInput.Length, layoutHeight);
                lastUpdate = DateTime.Now;
            }

            if (Console.KeyAvailable)
            {
                ConsoleKeyInfo keyInfo = Console.ReadKey(true);

                if (keyInfo.Key == ConsoleKey.Enter)
                {
                    if (currentInput == "/quit")
                    {
                        _irc?.Quit();
                        return AppScreen.Exit;
                    }

                    if (!string.IsNullOrWhiteSpace(currentInput))
                    {
                        // _irc?.SendMessage(currentInput);
                        // _messages.Add($"<{_nickname}> {currentInput}");
                    }

                    currentInput = "";
                }
                else if (keyInfo.Key == ConsoleKey.Backspace && currentInput.Length > 0)
                {
                    currentInput = currentInput[..^1];
                }
                else if (IsPrintableChar(keyInfo.KeyChar))
                {
                    currentInput += keyInfo.KeyChar;
                }
            }

            Thread.Sleep(50);
        }
    }

    static Layout CreateWindow()
    {
        return new Layout("Root")
            .SplitRows(
                new Layout("Top")
                    .Ratio(9)
                    .SplitColumns(
                        new Layout("Left").Ratio(3),
                        new Layout("Right")
                    ),
                new Layout("Bottom")
                    .Ratio(1)
            );
    }

    static Panel CreateChatPanel(List<string> messages)
    {
        var text = string.Join("\n", messages);

        return new Panel(Markup.Escape(text))
            .Expand()
            .Border(BoxBorder.Rounded)
            .BorderStyle(Color.Green)
            .Header("Chat");
    }

    static Panel CreateUserListPanel(List<string> users)
    {
        var text = string.Join("\n", users);

        return new Panel(text)
            .Expand()
            .Border(BoxBorder.Rounded)
            .BorderStyle(Color.Green)
            .Header("Users");
    }

    static Panel CreateInputPanel(string currentInput = "")
    {
        var inputText = $"[green]> [/]{currentInput}";

        return new Panel(new Markup(inputText))
            .Expand()
            .Border(BoxBorder.Rounded)
            .BorderStyle(Color.Green)
            .Header("Input");
    }
    
    static bool IsPrintableChar(char c)
    {
        return c >= 32 && c <= 126;
    }
}