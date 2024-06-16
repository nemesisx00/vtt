using System.Collections.Generic;

namespace Vtt.Network.Payload;

public interface Serializable
{
	public Dictionary<string, string> Serialize();
}
