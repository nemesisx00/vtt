using Godot;

namespace Vtt;

public partial class Token : CharacterBody2D
{
	private static class NodePaths
	{
		public static readonly NodePath Icon = new("%Icon");
		public static readonly NodePath NavigationAgent = new("%NavigationAgent2D");
		public static readonly NodePath Selection = new("%Selection");
	}
	
	public Vector2 Destination
	{
		get => navAgent.TargetPosition;
		set
		{
			if(Selected)
				navAgent.TargetPosition = value;
		}
	}
	
	public bool Selected => selectionSprite.Visible;
	
	[Export]
	private float speed = 500.0f;
	
	private NavigationAgent2D navAgent;
	private Sprite2D selectionSprite;
	
	public override void _InputEvent(Viewport viewport, InputEvent evt, int shapeIdx)
	{
		if(evt.IsActionPressed(Actions.Select))
			toggleSelection();
	}
	
	public override void _PhysicsProcess(double delta)
	{
		base._PhysicsProcess(delta);
		
		if(!navAgent.IsNavigationFinished())
		{
			var currentPosition = GlobalPosition;
			var nextPosition = navAgent.GetNextPathPosition();
			
			Velocity = currentPosition.DirectionTo(nextPosition) * speed;
			MoveAndSlide();
		}
	}
	
	public override void _Ready()
	{
		SetPhysicsProcess(false);
		
		navAgent = GetNode<NavigationAgent2D>(NodePaths.NavigationAgent);
		selectionSprite = GetNode<Sprite2D>(NodePaths.Selection);
		
		Callable.From(prepareNavigationAgent).CallDeferred();
	}
	
	private async void prepareNavigationAgent()
	{
		await ToSignal(GetTree(), SceneTree.SignalName.PhysicsFrame);
		navAgent.TargetPosition = GlobalPosition;
		SetPhysicsProcess(true);
	}
	
	private void toggleSelection()
	{
		if(selectionSprite.Visible)
			selectionSprite.Hide();
		else
			selectionSprite.Show();
	}
}
