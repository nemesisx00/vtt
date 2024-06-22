using Godot;

namespace Vtt;

public partial class Gameplay : Node2D
{
	private sealed class NodePaths
	{
		public static readonly NodePath Unit = new("Unit");
	}
	
	private Unit unit;
	
	public override void _UnhandledInput(InputEvent evt)
	{
		if(evt.IsActionPressed(Actions.Move) && evt is InputEventMouseButton iemb)
			unit.Destination = iemb.GlobalPosition;
	}
	
	public override void _Ready()
	{
		unit = GetNode<Unit>(NodePaths.Unit);
	}
}
