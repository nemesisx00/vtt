using Godot;
using System;
using Vtt.Network;
using Vtt.Network.Payload;

public partial class Ui : Node
{
	private sealed class NodePaths
	{
		public static readonly NodePath Connect = new("%Connect");
		public static readonly NodePath ClientNode = new("%Client");
		public static readonly NodePath Disconnect = new("%Disconnect");
		public static readonly NodePath Message = new("%Message");
		public static readonly NodePath Output = new("%Output");
	}
	
	private VttClient client;
	private RichTextLabel output;
	
	public override void _Ready()
	{
		client = GetNode<VttClient>(NodePaths.ClientNode);
		output = GetNode<RichTextLabel>(NodePaths.Output);
		
		GetNode<Button>(NodePaths.Connect).Pressed += () => client.ConnectSocket();
		GetNode<Button>(NodePaths.Disconnect).Pressed += () => client.DisconnectSocket();
		GetNode<Button>(NodePaths.Message).Pressed += () => client.SendMessage(new Broadcast { id = client.ClientId, text = "I clicked the test button!" });
	}
}
