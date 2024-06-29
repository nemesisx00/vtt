using Godot;
using Vtt.Board;
using Vtt.Network;

namespace Vtt;

public partial class Gameplay : Node2D
{
	private sealed class NodePaths
	{
		public static readonly NodePath Token = new("Token");
	}
	
	private VttClient client;
	private Token token;
	
	private PackedScene packedBoardScene;
	private BoardScene2D boardScene;
	
	public override void _ExitTree()
	{
		client.ReceivedScene2D -= handleReceived2dScene;
		
		base._ExitTree();
	}
	
	public override void _UnhandledInput(InputEvent evt)
	{
		if(evt.IsActionPressed(Actions.Move) && evt is InputEventMouseButton iemb)
			token.Destination = iemb.GlobalPosition;
	}
	
	public override void _Ready()
	{
		packedBoardScene = GD.Load<PackedScene>(Scenes.BoardScene2D);
		
		client = GetNode<VttClient>(VttClient.NodePath);
		token = GetNode<Token>(NodePaths.Token);
		
		client.ReceivedScene2D += handleReceived2dScene;
	}
	
	private void handleReceived2dScene(long height, long width, byte[] background)
	{
		if(packedBoardScene.CanInstantiate())
		{
			var image = Image.Create((int)width, (int)height, false, Image.Format.Rgba8);
			image.LoadPngFromBuffer(background);
			
			generateNewBoardScene2D(
				ImageTexture.CreateFromImage(image),
				new(650, 400)
			);
		}
	}
	
	private void generateNewBoardScene2D(Texture2D texture, Vector2 initialPosition = default)
	{
		if(texture is not null)
		{
			boardScene?.QueueFree();
			
			boardScene = packedBoardScene.Instantiate<BoardScene2D>();
			AddChild(boardScene);
			boardScene.BackgroundTexture = texture;
			boardScene.Position = initialPosition;
		}
	}
}
