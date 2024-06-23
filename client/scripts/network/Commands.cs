
namespace Vtt.Network;

public enum Commands
{
	None,
	
	AuthenticateRequest = 100,
	AuthenticateSend,
	AuthenticateFail,
	AuthenticateSuccess,
	
	BroadcastRequest = 200,
	BroadcastResponse,
	BroadcastGetRequest,
	
	Scene2DRequest = 300,
	Scene2DResponse,
}
