using Godot;
using System;
using System.Collections.Generic;
using System.Text;

namespace Vtt;

public partial class VttClient : Node
{
	private bool connected = false;
	private bool shouldConnect = true;
	private bool sent = false;
	private WebSocketPeer socket = new();
	
	public override void _Process(double delta)
	{
		if(!connected && shouldConnect)
			connectSocket();
		
		pollSocket();
	}
	
	public override void _Notification(int what)
	{
		switch((long)what)
		{
			case NotificationWMCloseRequest:
				disconnectSocket();
				break;
		}
	}
	
	private void disconnectSocket()
	{
		if(connected)
			socket.Close();
	}
	
	private void connectSocket()
	{
		var error = socket.ConnectToUrl("ws://127.0.0.1:7890");
		connected = Error.Ok == error;
		
		if(connected)
			shouldConnect = false;
	}
	
	private void pollSocket()
	{
		if(connected)
		{
			socket.Poll();
			switch(socket.GetReadyState())
			{
				case WebSocketPeer.State.Open:
					var count = socket.GetAvailablePacketCount();
					List<byte> data = new();
					for(int i = 0; i < count; i++)
					{
						var packet = socket.GetPacket();
						data.AddRange(packet);
					}
					
					if(data.Count > 0)
					{
						var text = Encoding.UTF8.GetString(data.ToArray());
						GD.Print("Response: '", text, "'");
					}
					
					if(!sent)
					{
						socket.SendText("Hello world!");
						sent = true;
					}
					break;
				
				case WebSocketPeer.State.Closing:
					break;
				
				case WebSocketPeer.State.Closed:
					var code = socket.GetCloseCode();
					var reason = socket.GetCloseReason();
					GD.Print(String.Format("Socket closed: {0} - {1}", code, reason));
					connected = false;
					break;
				
				case WebSocketPeer.State.Connecting:
					break;
			}
		}
	}
}
