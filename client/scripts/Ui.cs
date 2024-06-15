using System;
using System.Net;
using Godot;
using Vtt.Network;

namespace Vtt;

public partial class Ui : Node
{
	private sealed class NodePaths
	{
		public static readonly NodePath ClientId = new("%ClientId");
		public static readonly NodePath ClientNode = new("%Client");
		public static readonly NodePath Connect = new("%Connect");
		public static readonly NodePath ConnectUi = new("%ConnectUI");
		public static readonly NodePath ConnectedUI = new("%ConnectedUI");
		public static readonly NodePath Disconnect = new("%Disconnect");
		public static readonly NodePath IpAddress = new("%IpAddress");
		public static readonly NodePath Login = new("%Login");
		public static readonly NodePath LoginUi = new("%LoginUI");
		public static readonly NodePath Message = new("%Message");
		public static readonly NodePath Output = new("%Output");
		public static readonly NodePath Username = new("%Username");
		public static readonly NodePath UsernameInput = new("%UsernameInput");
	}
	
	private const int DefaultPort = 8080;
	
	private VBoxContainer connectUi;
	private VBoxContainer loginUi;
	private VBoxContainer connectedUi;
	
	private VttClient client;
	private Label clientId;
	private TextEdit ipAddress;
	private RichTextLabel output;
	private Label username;
	private TextEdit usernameInput;
	
	public override void _ExitTree()
	{
		client.DisplayMessage -= handleDisplayMessage;
		client.LoginResponse -= handleLoginResponse;
		client.SocketConnected -= handleSocketConnected;
		
		base._ExitTree();
	}
	
	public override void _Ready()
	{
		connectedUi = GetNode<VBoxContainer>(NodePaths.ConnectedUI);
		connectUi = GetNode<VBoxContainer>(NodePaths.ConnectUi);
		loginUi = GetNode<VBoxContainer>(NodePaths.LoginUi);
		
		client = GetNode<VttClient>(NodePaths.ClientNode);
		clientId = GetNode<Label>(NodePaths.ClientId);
		ipAddress = GetNode<TextEdit>(NodePaths.IpAddress);
		output = GetNode<RichTextLabel>(NodePaths.Output);
		username = GetNode<Label>(NodePaths.Username);
		usernameInput = GetNode<TextEdit>(NodePaths.UsernameInput);
		
		GetNode<Button>(NodePaths.Connect).Pressed += handleConnectButton;
		GetNode<Button>(NodePaths.Disconnect).Pressed += handleDisconnectButton;
		GetNode<Button>(NodePaths.Login).Pressed += handleLoginButton;
		GetNode<Button>(NodePaths.Message).Pressed += handleMessageButton;
		
		client.DisplayMessage += handleDisplayMessage;
		client.LoginResponse += handleLoginResponse;
		client.SocketConnected += handleSocketConnected;
		
		connectedUi.Hide();
		loginUi.Hide();
		connectUi.Show();
	}
	
	private void handleConnectButton()
	{
		if(ipAddress.Text.Length > 0)
		{
			var input = ipAddress.Text.Split(':');
			
			var ip = input[0] ?? string.Empty;
			if(input.Length < 2 || !int.TryParse(input[1], out int port))
				port = DefaultPort;
			
			try
			{
				var address = IPAddress.Parse(ip);
				var endpoint = new IPEndPoint(address, port);
				GD.Print("endpoint: ", endpoint.ToString());
				client.ConnectSocket(endpoint);
			}
			catch(FormatException fe)
			{
				GD.Print("FormatException: ", fe.ToString());
			}
			catch(Exception e)
			{
				GD.Print("Exception: ", e.ToString());
			}
		}
	}
	
	private void handleDisconnectButton()
	{
		client.DisconnectSocket();
		
		connectedUi.Hide();
		connectUi.Show();
		
		clientId.Text = string.Empty;
		username.Text = string.Empty;
		output.Clear();
	}
	
	private void handleDisplayMessage(string text)
		=> output.AddText($"\n{text}");
	
	private void handleLoginButton()
	{
		client.SendMessage(
			client.Status.id,
			Commands.AuthenticateSend,
			new() { { "name", usernameInput.Text } }
		);
	}
	
	private void handleLoginResponse(bool success)
	{
		if(success)
		{
			clientId.Text = client.Status.id.ToString();
			username.Text = usernameInput.Text;
			
			loginUi.Hide();
			connectUi.Hide();
			connectedUi.Show();
			
			usernameInput.Clear();
		}
		else
		{
			//TODO: alert user
		}
	}
	
	private void handleMessageButton()
	{
		client.SendMessage(
			client.Status.id,
			Commands.BroadcastSend,
			new() { { "text", "I clicked the test button!" } }
		);
	}
	
	private void handleSocketConnected()
	{
		connectedUi.Hide();
		connectUi.Hide();
		loginUi.Show();
		
		ipAddress.Clear();
	}
}
