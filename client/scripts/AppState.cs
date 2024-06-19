using Godot;
using Vtt.Network;

namespace Vtt;

public partial class AppState : Node
{
	private VttClient client;
	
	public override void _Notification(int what)
	{
		switch((long)what)
		{
			case NotificationWMCloseRequest:
				client.DisconnectSocket();
				GetTree().Quit();
				break;
		}
	}
	
	public override void _Ready()
	{
		GetTree().AutoAcceptQuit = false;
		
		client = GetNode<VttClient>(VttClient.NodePath);
	}
}
