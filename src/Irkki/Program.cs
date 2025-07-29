using Spectre.Console;

namespace Irkki;

enum AppScreen
{
    Start,
    Connect,
    Main,
    Exit
}

class Program
{
    static List<string> _messages = new();
    static List<string> _users = new();
    static string _nickname = "guest";
    static string _server = "localhost";
    static int _port = 6667;

    static void Main(string[] args)
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
                    break;
                case AppScreen.Main:
                    currentScreen = ShowMainScreen();
                    break;
                case AppScreen.Exit:
                    break;
            }
        }

        AnsiConsole.Clear();
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
                .Title("Choose an option:")
                .AddChoices("Start", "Exit"));

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
            new TextPrompt<string>("Enter your nickname:")
                .DefaultValue(_nickname));
             
        _server = AnsiConsole.Prompt(
            new TextPrompt<string>("Enter server address:")
                .DefaultValue(_server));

        _port = AnsiConsole.Prompt(
            new TextPrompt<int>("Enter server port:")
                .DefaultValue(_port));

        return AppScreen.Main;

    }

    static AppScreen ShowMainScreen()
    {
        var totalHeight = Console.WindowHeight;
        int layoutHeight = totalHeight - 2;

        while (true)
        {
            AnsiConsole.Clear();

            var layout = CreateWindow();
            layout["Left"].Update(CreateChatPanel(_messages));
            layout["Right"].Update(CreateUserListPanel(_users));
            layout["Bottom"].Update(CreateInputPanel());

            AnsiConsole.Write(layout);

            Console.SetCursorPosition(4, layoutHeight);
            string command = Console.ReadLine() ?? "";

            if (command == "/quit")
                return AppScreen.Exit;

            _messages.Add(command);
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

        return new Panel(text)
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

    static Panel CreateInputPanel()
    {
        return new Panel(new Markup("[bold green]> [/]"))
            .Expand()
            .Border(BoxBorder.Rounded)
            .BorderStyle(Color.Green)
            .Header("Input");
    }
}
