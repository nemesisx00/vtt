using System;
using System.Net;
using Godot;
using Vtt.Network;
using Vtt.Network.Payload;

namespace Vtt;

public partial class Ui : Node
{
	private sealed class NodePaths
	{
		public static readonly NodePath ClientId = new("%ClientId");
		public static readonly NodePath Connect = new("%Connect");
		public static readonly NodePath ConnectUi = new("%ConnectUI");
		public static readonly NodePath ConnectedUI = new("%ConnectedUI");
		public static readonly NodePath Disconnect = new("%Disconnect");
		public static readonly NodePath IpAddress = new("%IpAddress");
		public static readonly NodePath Login = new("%Login");
		public static readonly NodePath LoginUi = new("%LoginUI");
		public static readonly NodePath Message = new("%Message");
		public static readonly NodePath MessageText = new("%MessageText");
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
	private LineEdit ipAddress;
	private LineEdit messageText;
	private RichTextLabel output;
	private Label username;
	private LineEdit usernameInput;
	
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
		
		client = GetNode<VttClient>(VttClient.NodePath);
		clientId = GetNode<Label>(NodePaths.ClientId);
		ipAddress = GetNode<LineEdit>(NodePaths.IpAddress);
		messageText = GetNode<LineEdit>(NodePaths.MessageText);
		output = GetNode<RichTextLabel>(NodePaths.Output);
		username = GetNode<Label>(NodePaths.Username);
		usernameInput = GetNode<LineEdit>(NodePaths.UsernameInput);
		
		GetNode<Button>(NodePaths.Connect).Pressed += handleConnectButton;
		GetNode<Button>(NodePaths.Disconnect).Pressed += handleDisconnectButton;
		GetNode<Button>(NodePaths.Login).Pressed += handleLoginButton;
		GetNode<Button>(NodePaths.Message).Pressed += handleMessageButton;
		
		ipAddress.TextSubmitted += _ => handleConnectButton();
		messageText.TextSubmitted += _ => handleMessageButton();
		usernameInput.TextSubmitted += _ => handleLoginButton();
		
		client.DisplayMessage += handleDisplayMessage;
		client.LoginResponse += handleLoginResponse;
		client.SocketConnected += handleSocketConnected;
		
		connectedUi.Hide();
		loginUi.Hide();
		connectUi.Show();
		
		ipAddress.GrabFocus();
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
				client.ConnectSocket(endpoint);
			}
			catch (Exception)
			{
				//TODO: Inform user of error
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
		output.Text = string.Empty;
		output.Clear();
		
		ipAddress.GrabFocus();
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
			messageText.GrabFocus();
		}
		else
		{
			//TODO: alert user
		}
	}
	
	private void handleMessageButton()
	{
		if(!string.IsNullOrEmpty(messageText.Text))
		{
			var text = messageText.Text;
			//TODO: Sanitize input
			messageText.Clear();
			
			client.SendMessage(
				client.Status.id,
				Commands.BroadcastSend,
				new BroadcastData(text)
			);
		}
	}
	
	private void handleSocketConnected()
	{
		connectedUi.Hide();
		connectUi.Hide();
		loginUi.Show();
		
		ipAddress.Clear();
		usernameInput.GrabFocus();
	}
}
