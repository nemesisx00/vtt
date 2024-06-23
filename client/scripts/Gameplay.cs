using Godot;
using Vtt.Board;

namespace Vtt;

public partial class Gameplay : Node2D
{
	private sealed class NodePaths
	{
		public static readonly NodePath Token = new("Token");
	}
	
	private Token token;
	
	public override void _UnhandledInput(InputEvent evt)
	{
		if(evt.IsActionPressed(Actions.Move) && evt is InputEventMouseButton iemb)
			token.Destination = iemb.GlobalPosition;
	}
	
	public override void _Ready()
	{
		token = GetNode<Token>(NodePaths.Token);
	}
}
