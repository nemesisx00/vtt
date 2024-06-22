using System;
using System.Net;
using Godot;
using Vtt.Network;
using Vtt.Network.Payload;

namespace Vtt;

public partial class MainMenu : MarginContainer
{
	private sealed class NodePaths
	{
		public static readonly NodePath Connect = new("%Connect");
		public static readonly NodePath ConnectUi = new("%ConnectUI");
		public static readonly NodePath IpAddress = new("%IpAddress");
		public static readonly NodePath Login = new("%Login");
		public static readonly NodePath LoginUi = new("%LoginUI");
		public static readonly NodePath UsernameInput = new("%UsernameInput");
		public static readonly NodePath Quit = new("%Quit");
	}
	
	private const int DefaultPort = 8080;
	
	private VBoxContainer connectUi;
	private VBoxContainer loginUi;
	
	private VttClient client;
	private LineEdit ipAddress;
	private LineEdit usernameInput;
	
	public override void _ExitTree()
	{
		client.LoginResponse -= handleLoginResponse;
		client.SocketConnected -= handleSocketConnected;
		
		base._ExitTree();
	}
	
	public override void _Ready()
	{
		connectUi = GetNode<VBoxContainer>(NodePaths.ConnectUi);
		loginUi = GetNode<VBoxContainer>(NodePaths.LoginUi);
		
		client = GetNode<VttClient>(VttClient.NodePath);
		ipAddress = GetNode<LineEdit>(NodePaths.IpAddress);
		usernameInput = GetNode<LineEdit>(NodePaths.UsernameInput);
		
		GetNode<Button>(NodePaths.Connect).Pressed += handleConnectButton;
		GetNode<Button>(NodePaths.Login).Pressed += handleLoginButton;
		GetNode<Button>(NodePaths.Quit).Pressed += handleQuit;
		
		ipAddress.TextSubmitted += _ => handleConnectButton();
		usernameInput.TextSubmitted += _ => handleLoginButton();
		
		client.LoginResponse += handleLoginResponse;
		client.SocketConnected += handleSocketConnected;
		
		loginUi.Hide();
		connectUi.Show();
		
		ipAddress.GrabFocus();
	}
	
	private void handleConnectButton()
	{
		if(!string.IsNullOrEmpty(ipAddress.Text))
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
	
	private void handleLoginButton()
	{
		if(!string.IsNullOrEmpty(usernameInput.Text))
		{
			client.SendMessage(
				client.Status.id,
				Commands.AuthenticateSend,
				new AuthenticationPayload(usernameInput.Text)
			);
		}
	}
	
	private void handleLoginResponse(bool success)
	{
		if(success)
		{
			loginUi.Hide();
			connectUi.Hide();
			
			usernameInput.Clear();
			
			GetTree().ChangeSceneToFile(Scenes.Gameplay);
		}
		else
		{
			//TODO: alert user
		}
	}
	
	private void handleQuit()
		=> GetNode<AppState>(AppState.NodePath)
			.Notification((int)NotificationWMCloseRequest);
	
	private void handleSocketConnected()
	{
		connectUi.Hide();
		loginUi.Show();
		
		ipAddress.Clear();
		usernameInput.GrabFocus();
	}
}
