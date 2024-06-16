using System.Collections.Generic;

namespace Vtt.Network.Payload;

public class Command() : Message
{
	public long Id { get; set; } = -1;
	public Commands Type { get; set; }
	public Dictionary<string, string> Data { get; set; } = [];
	
	public AuthenticationData ParseAuthenticationData() => new(Data);
	public BroadcastData ParseBroadcastData() => BroadcastData.Deserialize(Data);
}
