using System.Collections.Generic;
using System.Xml;

namespace Vtt.Network.Payload;

public class Command() : Message
{
	public long Id { get; set; } = -1;
	public Commands Type { get; set; }
	public Dictionary<string, string> Data { get; set; } = [];
	
	public AuthenticationData AuthenticationData() => new(Data);
	public BroadcastData BroadcastData() => new(Data);
}

public struct AuthenticationData(Dictionary<string, string> data)
{
	public readonly long ClientId => long.TryParse(data["clientId"], out long newId)
		? newId
		: -1;
}

public struct BroadcastData(Dictionary<string, string> data)
{
	public readonly string Text => data["text"];
}
