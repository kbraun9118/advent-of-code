public class Graph
{
    public Cave Start { get; init; }

    public Graph(string[] lines)
    {
        Start = new Cave("start");
        var connections = lines.Select(l => l.Split("-")).Select(s => (s[0], s[1])).ToList();

        var nodesToAdd = new List<Cave>() { Start };

        while (connections.Count > 0)
        {
            var cloned = nodesToAdd.ToList();
            nodesToAdd.Clear();

            foreach (var node in cloned)
            {
                var linesWithNode = connections.Where(c => c.Item1 == node.Name || c.Item2 == node.Name).ToList();
                foreach (var line in linesWithNode)
                {
                    connections.Remove(line);
                    if (line.Item1 == node.Name)
                    {
                        var next = Start.Find(line.Item2) ?? new Cave(line.Item2);
                        nodesToAdd.Add(next);
                        node.Connect(next);
                    }
                    else
                    {
                        var next = Start.Find(line.Item1) ?? new Cave(line.Item1);
                        nodesToAdd.Add(next);
                        node.Connect(next);
                    }
                }
            }
        }
    }

    public List<List<string>> Paths(bool canVisitSmall = false)
    {
        return PathsFromNode(Start, new(), !canVisitSmall).Select(p => p.Select(c => c.Name).ToList()).ToList();
    }

    private List<List<Cave>> PathsFromNode(Cave from, List<Cave> previous, bool hasVisitedSmall)
    {
        if (from.End)
        {
            return new() { previous.Append(from).ToList() };
        }
        else if (from.SmallCave && previous.Contains(from) && hasVisitedSmall)
        {
            return new();
        }
        else
        {
            return from.Connections
                .Where(c => !c.Start)
                .SelectMany(c => PathsFromNode(c, previous.Append(from).ToList(), from.SmallCave && previous.Contains(from) ? true : hasVisitedSmall))
                .ToList();
        }
    }
}

public class Cave
{
    public string Name { get; init; }
    public HashSet<Cave> Connections { get; init; } = new();
    public bool SmallCave => !Start && !End && Name.ToCharArray().All(Char.IsLower);
    public bool Start => Name == "start";
    public bool End => Name == "end";

    public Cave(string name)
    {
        Name = name;
    }

    public void Connect(Cave node)
    {
        this.Connections.Add(node);
        node.Connections.Add(this);
    }

    public Cave? Find(Cave node)
    {
        return Find(node.Name);
    }

    public Cave? Find(string name)
    {
        if (name == "start") return this;
        Cave? returned = null;
        var visited = new List<string>() { Name };
        var toCheck = Connections.ToList();
        while (returned is null && toCheck.Count > 0)
        {
            if (toCheck.Select(n => n.Name).Contains(name))
            {
                return toCheck.First(n => n.Name == name);
            }
            else
            {
                toCheck = toCheck.SelectMany(n => n.Connections)
                    .Where(n => !visited.Contains(n.Name))
                    .ToList();
                visited.AddRange(toCheck.Select(n => n.Name));
            }
        }

        return returned;
    }

    public override bool Equals(object? obj)
    {
        return obj is Cave node &&
               Name == node.Name;
    }

    public override int GetHashCode()
    {
        return HashCode.Combine(Name);
    }

    public static bool operator ==(Cave? left, Cave? right)
    {
        return left is not null
            && right is not null
            && left.Name == right.Name;
    }

    public static bool operator !=(Cave? left, Cave? right)
    {
        return !(left == right);
    }
}