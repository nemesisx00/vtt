using Godot;
using System.Collections.Generic;
using System.Text;
using System.Text.Json;
using System.Globalization;
using Vtt.Network.Payload;

namespace Vtt.Network;

public partial class VttClient : Node
{
	[Signal]
	public delegate void SocketConnectedEventHandler();
	
	[Export]
	public long ClientId { get; set; }
	
	private bool connected = false;
	private WebSocketPeer socket = new();
	
	public override void _Notification(int what)
	{
		switch((long)what)
		{
			case NotificationWMCloseRequest:
				DisconnectSocket();
				break;
		}
	}
	
	public override void _Process(double delta)
	{
		pollSocket();
	}
	
	public override void _Ready()
	{
		ClientId = Multiplayer.GetUniqueId();
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
	
	public void SendMessage(long id, Commands type) => SendMessage(id, type, []);
	
	public void SendMessage(long id, Commands type, Dictionary<string, string> data)
		=> SendMessage(new Command { Id = id, Type = type, Data = data, });
	
	public void SendMessage(Command message)
	{
		if(connected)
			socket.SendText(JsonSerializer.Serialize(message));
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
					
					List<byte> data = [];
					for(int i = 0; i < count; i++)
					{
						var packet = socket.GetPacket();
						data.AddRange(packet);
					}
					
					if(data.Count > 0)
						parseCommand(Encoding.UTF8.GetString(data.ToArray()));
					break;
				
				case WebSocketPeer.State.Closing:
					break;
				
				case WebSocketPeer.State.Closed:
					GD.Print(string.Format("Socket closed: {0} - {1}", socket.GetCloseCode(), socket.GetCloseReason()));
					connected = false;
					break;
				
				case WebSocketPeer.State.Connecting:
					break;
			}
		}
	}
	
	private void parseCommand(string data)
	{
		var list = JsonSerializer.Deserialize<List<Command>>(data);
		
		foreach(var command in list)
		{
			switch(command.Type)
			{
				case Commands.AuthenticateRequest:
					GD.Print("Authentication requested by server!");
					break;
				
				case Commands.AuthenticateFail:
					GD.Print("Authentication failed!");
					break;
				
				case Commands.AuthenticateSuccess:
					GD.Print("Authentication succeeded!");
					if(long.TryParse(command.Data["clientId"], out long newId))
					{
						ClientId = newId;
						GD.Print("Current Client ID: ", ClientId);
					}
					break;
				
				case Commands.BroadcastReceive:
					GD.Print("Broadcast: ", command.Data["text"]);
					break;
			}
		}
	}
}
