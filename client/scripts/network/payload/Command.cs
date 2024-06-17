using System;
using System.Collections.Generic;

namespace Vtt.Network.Payload;

public class Command() : Message
{
	public Dictionary<string, string> Data { get; set; } = [];
	public long Id { get; set; } = -1;
	public long Timestamp { get; set; } = DateTimeOffset.UtcNow.ToUnixTimeSeconds();
	public Commands Type { get; set; }
	
	public DateTimeOffset ParseTimestamp() => DateTimeOffset.FromUnixTimeSeconds(Timestamp);
	public AuthenticationData ParseAuthenticationData() => new(Data);
	public BroadcastData ParseBroadcastData() => BroadcastData.Deserialize(Data);
}
