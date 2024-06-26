using System;
using System.Collections.Generic;

namespace Vtt.Network.Payload;

public class Command() : Message
{
	public Dictionary<string, byte[]> BinaryData { get; set; } = [];
	public Dictionary<string, string> Data { get; set; } = [];
	public long Id { get; set; } = -1;
	public long Timestamp { get; set; } = DateTimeOffset.UtcNow.ToUnixTimeSeconds();
	public Commands Type { get; set; }
	
	public DateTimeOffset ParseTimestamp() => DateTimeOffset.FromUnixTimeSeconds(Timestamp);
	public AuthenticationData ParseAuthenticationData() => AuthenticationData.Deserialize(Data);
	public BroadcastData ParseBroadcastData() => BroadcastData.Deserialize(Data);
	public Scene2DData ParseScene2DData() => Scene2DData.Deserialize(Data, BinaryData);
}
