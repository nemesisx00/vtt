
namespace Vtt.Network.Payload;

public class Broadcast : ClientIdentity, Message
{
	public string text { get; set; }
}
