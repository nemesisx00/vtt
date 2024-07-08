using System.Linq;
using Godot;
using Vtt.Board;
using Vtt.Extensions;
using Vtt.Network;

namespace Vtt;

public partial class Gameplay : Node2D
{
	private sealed class NodePaths
	{
		public static readonly NodePath Camera = new("%Camera2D");
		public static readonly NodePath GameplayUi = new("%GameplayUI");
	}
	
	[Export]
	private float scrollSpeed = 0.01f;
	[Export]
	private float zoomMax = 2.0f;
	[Export]
	private float zoomMin = 0.1f;
	[Export]
	private float zoomStep = 0.1f;
	
	private Camera2D camera;
	private bool canMove;
	private VttClient client;
	private PackedScene packedBoardScene;
	private BoardScene2D boardScene;
	
	public override void _ExitTree()
	{
		client.ReceivedScene2D -= handleReceived2dScene;
		
		GetNode<GameplayUI>(NodePaths.GameplayUi).AddTokenRequest -= handleAddTokenRequest;
		
		base._ExitTree();
	}
	
	public override void _Ready()
	{
		packedBoardScene = GD.Load<PackedScene>(Scenes.BoardScene2D);
		
		camera = GetNode<Camera2D>(NodePaths.Camera);
		client = GetNode<VttClient>(VttClient.NodePath);
		
		client.ReceivedScene2D += handleReceived2dScene;
		GetNode<GameplayUI>(NodePaths.GameplayUi).AddTokenRequest += handleAddTokenRequest;
	}
	
	public override void _Input(InputEvent evt)
	{
		if(evt.IsActionPressed(Actions.MoveCamera))
		{
			canMove = true;
			Input.MouseMode = Input.MouseModeEnum.Captured;
		}
		
		if(evt.IsActionReleased(Actions.MoveCamera))
		{
			canMove = false;
			Input.MouseMode = Input.MouseModeEnum.Visible;
		}
		
		if(canMove && evt is InputEventMouseMotion iemm)
			camera.GlobalPosition += iemm.Velocity * -scrollSpeed;
		
		if(evt.GetActionStrength(Actions.ZoomIn) > 0)
		{
			camera.Zoom = (camera.Zoom + (camera.Zoom * zoomStep)).Clamp(zoomMin, zoomMax);
			camera.Scale = new(1 / camera.Zoom.X, 1 / camera.Zoom.Y);
		}
		
		if(evt.GetActionStrength(Actions.ZoomOut) > 0)
		{
			camera.Zoom = (camera.Zoom - (camera.Zoom * zoomStep)).Clamp(zoomMin, zoomMax);
			camera.Scale = new(1 / camera.Zoom.X, 1 / camera.Zoom.Y);
		}
	}
	
	public override void _UnhandledInput(InputEvent evt)
	{
		if(evt.IsActionPressed(Actions.Move) && evt is InputEventMouseButton iemb)
		{
			boardScene.Tokens
				.Where(t => t.Selected)
				.ToList()
				.ForEach(token => token.Destination = iemb.GlobalPosition);
		}
	}
	
	private void handleAddTokenRequest() => boardScene?.AddToken();
	
	private void handleReceived2dScene(long height, long width, byte[] background)
	{
		if(packedBoardScene.CanInstantiate())
		{
			var image = Image.Create((int)width, (int)height, false, Image.Format.Rgba8);
			image.LoadPngFromBuffer(background);
			
			generateNewBoardScene2D(
				ImageTexture.CreateFromImage(image),
				new((int)width, (int)height)
			);
		}
	}
	
	private void generateNewBoardScene2D(Texture2D texture, Vector2 size, Vector2 initialPosition = default)
	{
		if(texture is not null)
		{
			boardScene?.QueueFree();
			
			boardScene = packedBoardScene.Instantiate<BoardScene2D>();
			AddChild(boardScene);
			boardScene.BackgroundTexture = texture;
			boardScene.Position = initialPosition;
			boardScene.GenerateGrid(size);
		}
	}
}
