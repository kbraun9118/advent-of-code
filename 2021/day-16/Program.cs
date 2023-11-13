public class Program
{

    public static void Main(string[] args)
    {
        var file = File.ReadAllLines(@"./input.txt")[0];

        var binFile = string
            .Join("", file.ToCharArray()
                .Select(c => c.ToString())
                .Select(c => Convert
                    .ToString(int.Parse(c.ToString(), System.Globalization.NumberStyles.HexNumber), 2)
                    .PadLeft(4, '0')
                )
            );

        var tail = binFile;
        var packets = new List<Packet>();

        while (!tail.All(c => c == '0'))
        {
            var response = DecodeNext(tail);
            tail = response.Tail;
            packets.Add(response.Current);
        }

        Console.WriteLine($"Part One: {packets.Select(p => p.SumVersions()).Sum()}");
        Console.WriteLine($"Part Two: {packets[0].Execute()}");
    }

    public static (Packet Current, string Tail) DecodeNext(string input)
    {
        var version = input[0..3].FromBinString();
        var typeId = input[3..6].FromBinString();
        input = input.Substring(6);
        if (typeId == 4)
        {
            var literalStr = "";
            while (input[0] == '1')
            {
                literalStr += input[1..5];
                input = input.Substring(5);
            }
            literalStr += input[1..5];
            var packet = new LiteralPacket(version, typeId, literalStr.FromBinString());
            input = input.Substring(5);
            return (packet, input);
        }
        else
        {
            if (input[0] == '0')
            {

                var totalLenghtInBits = input[1..16].FromBinString();
                var subPackets = new List<Packet>();
                input = input.Substring(16);
                var subPacketsInput = input[0..(int)totalLenghtInBits];
                input = input.Substring((int)totalLenghtInBits);
                while (subPacketsInput.Length > 0)
                {
                    var output = DecodeNext(subPacketsInput);
                    subPacketsInput = output.Tail;
                    subPackets.Add(output.Current);
                }
                return (new OperatorPacket(version, typeId, subPackets), input);

            }
            else
            {
                var numberOfPackets = input[1..12].FromBinString();
                var subPackets = new List<Packet>();
                input = input.Substring(12);
                for (int i = 0; i < numberOfPackets; i++)
                {
                    var output = DecodeNext(input);
                    input = output.Tail;
                    subPackets.Add(output.Current);
                }
                return (new OperatorPacket(version, typeId, subPackets), input);
            }
        }
    }
}

public static class Extensions
{

    public static long FromBinString(this string input)
    {
        return Convert.ToInt64(input, 2);
    }
}

public abstract class Packet
{
    protected long VersionId { get; set; }
    protected long TypeId { get; set; }

    protected Packet(long version, long typeId)
    {
        TypeId = typeId;
        VersionId = version;
    }

    public abstract long SumVersions();

    public abstract long Execute();
}

public class LiteralPacket : Packet
{
    long Value { get; set; }

    public LiteralPacket(long version, long typeId, long value) : base(version, typeId)
    {
        Value = value;
    }

    public override long SumVersions()
    {
        return VersionId;
    }

    public override long Execute()
    {
        return Value;
    }
}

public class OperatorPacket : Packet
{
    List<Packet> SubPackets { get; init; }

    public OperatorPacket(long version, long typeId, List<Packet> subPackets) : base(version, typeId)
    {
        SubPackets = subPackets;
    }

    public override long SumVersions()
    {
        return SubPackets.Select(p => p.SumVersions()).Sum() + VersionId;
    }

    public override long Execute()
    {
        return TypeId switch
        {
            0 => SubPackets.Select(p => p.Execute()).Sum(),
            1 => SubPackets.Select(p => p.Execute()).Aggregate(1L, (acc, next) => acc * next),
            2 => SubPackets.Select(p => p.Execute()).Min(),
            3 => SubPackets.Select(p => p.Execute()).Max(),
            5 => SubPackets[0].Execute() > SubPackets[1].Execute() ? 1 : 0,
            6 => SubPackets[0].Execute() < SubPackets[1].Execute() ? 1 : 0,
            7 => SubPackets[0].Execute() == SubPackets[1].Execute() ? 1 : 0,
            _ => throw new ArgumentException($"Invalid Type Id: {TypeId}")
        };
    }
}