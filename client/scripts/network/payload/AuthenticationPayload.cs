using System.Collections.Generic;

namespace Vtt.Network.Payload;

public struct AuthenticationPayload(string username) : Serializable
{
	public readonly string Name => username;
	
	public readonly Dictionary<string, string> Serialize()
		=> new() { { "name", Name } };
}
