using System.Collections.Generic;

namespace Vtt.Network.Payload;

public struct AuthenticationData(long clientId, string username)
{
	public static AuthenticationData Deserialize(Dictionary<string, string> data)
		=> new(
			long.TryParse(data["clientId"], out long newId)
				? newId
				: -1,
			data["username"] ?? string.Empty
		);
	
	public readonly long ClientId => clientId;
	public readonly string Username => username;
}
