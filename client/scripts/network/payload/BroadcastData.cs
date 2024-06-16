using System.Collections.Generic;

namespace Vtt.Network.Payload;

public struct BroadcastData(string text) : Serializable
{
	public static BroadcastData Deserialize(Dictionary<string, string> data)
		=> new(data["text"]);
	
	public readonly string Text => text;
	
	public readonly Dictionary<string, string> Serialize()
		=> new() { { "text", Text } };
}
