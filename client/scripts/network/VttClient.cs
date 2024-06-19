using Godot;
using System.Collections.Generic;
using System.Net;
using System.Text;
using System.Text.Json;
using Vtt.Network.Payload;

namespace Vtt.Network;

public partial class VttClient : Node
{
	[Signal]
	public delegate void SocketConnectedEventHandler();
	[Signal]
	public delegate void LoginResponseEventHandler(bool success);
	[Signal]
	public delegate void DisplayMessageEventHandler(string text);
	
	public static readonly NodePath NodePath = new("/root/VttClient");
	
	public ClientStatus Status => status;
	
	private ClientStatus status;
	private WebSocketPeer socket = new();
	
	public override void _Process(double delta) => pollSocket();
	public override void _Ready() => status.id = Multiplayer.GetUniqueId();
	
	public void ConnectSocket(IPEndPoint ip)
	{
		if(!status.connected)
		{
			var error = socket.ConnectToUrl($"ws://{ip}");
			status.connected = Error.Ok == error;
			
			if(status.connected)
				EmitSignal(SignalName.SocketConnected);
		}
	}
	
	public void DisconnectSocket()
	{
		if(status.connected)
			socket.Close();
	}
	
	public void SendMessage(long id, Commands type) => SendMessage(id, type, []);
	
	public void SendMessage<T>(long id, Commands type, T data)
			where T: Serializable
		=> SendMessage(id, type, data.Serialize());
	
	public void SendMessage(long id, Commands type, Dictionary<string, string> data)
		=> SendMessage(new Command { Id = id, Type = type, Data = data, });
	
	public void SendMessage(Command message)
	{
		if(status.connected)
			socket.SendText(JsonSerializer.Serialize(message));
	}
	
	private void pollSocket()
	{
		if(status.connected)
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
				
				case WebSocketPeer.State.Closed:
					GD.Print(string.Format("Socket closed: {0} - {1}", socket.GetCloseCode(), socket.GetCloseReason()));
					status.Disconnect();
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
				case Commands.AuthenticateFail:
					EmitSignal(SignalName.LoginResponse, false);
					break;
				
				case Commands.AuthenticateSuccess:
					var ad = command.ParseAuthenticationData();
					status.id = ad.ClientId;
					status.loggedIn = true;
					EmitSignal(SignalName.LoginResponse, true);
					break;
				
				case Commands.BroadcastReceive:
					var bd = command.ParseBroadcastData();
					EmitSignal(SignalName.DisplayMessage, bd.Text);
					break;
			}
		}
	}
}
