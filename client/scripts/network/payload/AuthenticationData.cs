using System.Collections.Generic;

namespace Vtt.Network.Payload;

public struct AuthenticationData(Dictionary<string, string> data)
{
	public readonly long ClientId => long.TryParse(data["clientId"], out long newId)
		? newId
		: -1;
}
