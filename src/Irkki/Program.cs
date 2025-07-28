using Spectre.Console;

namespace Irkki;

class Program
{
    static List<string> messages = new();
    static List<string> users = new();

    static void Main(string[] args)
    {        
        var totalHeight = Console.WindowHeight;
        int layoutHeight = totalHeight - 2;

        while (true)
        {
            AnsiConsole.Clear();

            var layout = CreateWindow();
            layout["Left"].Update(CreateChatPanel(messages));
            layout["Right"].Update(CreateUserListPanel(users));
            layout["Bottom"].Update(CreateInputPanel());

            AnsiConsole.Write(layout);

            Console.SetCursorPosition(4, layoutHeight);
            string command = Console.ReadLine() ?? "";

            if (command == "/quit")
                break;

            messages.Add(command);
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
