
namespace Vtt.Network;

public enum Commands
{
	None,
	
	AuthenticateRequest = 100,
	AuthenticateSend,
	AuthenticateFail,
	AuthenticateSuccess,
	
	BroadcastReceive = 200,
	BroadcastSend,
}
