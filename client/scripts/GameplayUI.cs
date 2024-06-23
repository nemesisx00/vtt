using System;
using Godot;
using Vtt.Network;
using Vtt.Network.Payload;

namespace Vtt;

public partial class GameplayUI : MarginContainer
{
	private sealed class NodePaths
	{
		public static readonly NodePath ClientId = new("%ClientId");
		public static readonly NodePath Disconnect = new("%Disconnect");
		public static readonly NodePath Message = new("%Message");
		public static readonly NodePath MessageText = new("%MessageText");
		public static readonly NodePath Output = new("%Output");
		public static readonly NodePath Username = new("%Username");
	}
	
	private VttClient client;
	private Label clientId;
	private LineEdit messageText;
	private RichTextLabel output;
	private Label username;
	
	public override void _ExitTree()
	{
		client.DisplayMessage -= handleDisplayMessage;
		
		base._ExitTree();
	}
	
	public override void _Ready()
	{
		client = GetNode<VttClient>(VttClient.NodePath);
		clientId = GetNode<Label>(NodePaths.ClientId);
		messageText = GetNode<LineEdit>(NodePaths.MessageText);
		output = GetNode<RichTextLabel>(NodePaths.Output);
		username = GetNode<Label>(NodePaths.Username);
		
		GetNode<Button>(NodePaths.Disconnect).Pressed += handleDisconnectButton;
		GetNode<Button>(NodePaths.Message).Pressed += handleMessageButton;
		
		messageText.TextSubmitted += _ => handleMessageButton();
		
		client.DisplayMessage += handleDisplayMessage;
		
		
		//Request all broadcasts from the past 24 hours
		client.SendMessage(
			client.Status.id,
			Commands.BroadcastGetRequest,
			new BroadcastGetRequestData(
				DateTimeOffset.UtcNow.AddDays(-1).ToUnixTimeSeconds(),
				DateTimeOffset.UtcNow.ToUnixTimeSeconds()
			)
		);
		
		clientId.Text = client.Status.id.ToString();
		username.Text = client.Status.username;
		
		messageText.GrabFocus();
	}
	
	private void handleDisconnectButton()
	{
		client.DisconnectSocket();
		
		GetTree().ChangeSceneToFile(Scenes.MainMenu);
	}
	
	private void handleDisplayMessage(string text)
		=> output.AddText($"\n{text}");
	
	private void handleMessageButton()
	{
		if(!string.IsNullOrEmpty(messageText.Text))
		{
			var text = messageText.Text;
			//TODO: Sanitize input
			messageText.Clear();
			
			client.SendMessage(
				client.Status.id,
				Commands.BroadcastRequest,
				new BroadcastData(text)
			);
		}
	}
}
