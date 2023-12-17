using Godot;
using System;
using System.Collections.Generic;
using System.Text;
using System.Text.Json;
using Vtt.Network.Payload;

namespace Vtt.Network;

public partial class VttClient : Node
{
	[Signal]
	public delegate void SocketConnectedEventHandler();
	
	[Export]
	public long ClientId { get; set; }
	
	private bool connected = false;
	private bool idSent = false;
	private WebSocketPeer socket = new();
	
	public override void _Process(double delta)
	{
		pollSocket();
	}
	
	public override void _Ready()
	{
		ClientId = Multiplayer.GetUniqueId();
	}
	
	public override void _Notification(int what)
	{
		switch((long)what)
		{
			case NotificationWMCloseRequest:
				DisconnectSocket();
				break;
		}
	}
	
	public void ConnectSocket()
	{
		if(!connected)
		{
			var error = socket.ConnectToUrl("ws://127.0.0.1:7890");
			connected = Error.Ok == error;
		}
	}
	
	public void DisconnectSocket()
	{
		if(connected)
			socket.Close();
	}
	
	public void SendMessage<T>(T message)
		where T: Message
	{
		if(connected)
		{
			var json = JsonSerializer.Serialize(message);
			var payload = String.Format("{0}{1}", message.GetType().Name, json);
			socket.SendText(payload);
		}
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
						GD.Print(text);
					}
					
					if(!idSent)
					{
						SendMessage(new ClientIdentity { id = ClientId });
						idSent = true;
					}
					break;
				
				case WebSocketPeer.State.Closing:
					break;
				
				case WebSocketPeer.State.Closed:
					GD.Print(String.Format("Socket closed: {0} - {1}", socket.GetCloseCode(), socket.GetCloseReason()));
					connected = false;
					break;
				
				case WebSocketPeer.State.Connecting:
					break;
			}
		}
	}
}
