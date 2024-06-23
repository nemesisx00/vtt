using System.Collections.Generic;

namespace Vtt.Network.Payload;

public struct BroadcastGetRequestData(long start, long end) : Serializable
{
	public readonly long End => end;
	public readonly long Start => start;
	
	public readonly Dictionary<string, string> Serialize()
		=> new()
			{
				{ "start", Start.ToString() },
				{ "end", End.ToString() },
			};
}
