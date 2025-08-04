using Godot;

namespace Vtt.Board;

public partial class GridTile : Node2D
{
	private static class NodePaths
	{
		public static readonly NodePath HoverMarker = new("%HoverMarker");
		public static readonly NodePath Sprite = new("%Sprite");
	}
	
	[Signal]
	public delegate void AnnounceGridTileCenterEventHandler(Vector2 center);

	private MeshInstance2D hoverMarker;
	private Sprite2D sprite;
	private bool tileHovered;
	
	public override void _Input(InputEvent evt)
	{
		if(evt is InputEventMouseMotion iemm)
			tileHovered = sprite.GetRect().HasPoint(sprite.ToLocal(iemm.GlobalPosition));

		if (tileHovered && evt is InputEventMouseButton iemb
			&& (
				iemb.IsActionPressed(Actions.Select)
				|| iemb.IsActionPressed(Actions.Move)
			)
		)
		{
			handleClick();
		}
	}
	
	public override void _PhysicsProcess(double delta)
	{
		if(tileHovered)
		{
			if(!hoverMarker.Visible)
				hoverMarker.Show();
		}
		else
		{
			if(hoverMarker.Visible)
				hoverMarker.Hide();
		}
	}
	
	public override void _Ready()
	{
		hoverMarker = GetNode<MeshInstance2D>(NodePaths.HoverMarker);
		sprite = GetNode<Sprite2D>(NodePaths.Sprite);
	}
	
	private void handleClick()
	{
		EmitSignal(SignalName.AnnounceGridTileCenter, GlobalPosition);
	}
}
