using Godot;

namespace Vtt.Board;

public partial class Token : CharacterBody2D
{
	private static class NodePaths
	{
		public static readonly NodePath Icon = new("%Icon");
		public static readonly NodePath NavigationAgent = new("%NavigationAgent2D");
		public static readonly NodePath Selection = new("%Selection");
	}
	
	[Signal]
	public delegate void ReachedDestinationEventHandler(long id, Vector2 position);
	
	[Signal]
	public delegate void TokenDeselectedEventHandler(Token self);
	
	[Signal]
	public delegate void TokenSelectedEventHandler(Token self);
	
	public Vector2 Destination
	{
		get => navAgent.TargetPosition;
		set
		{
			if(Selected)
				navAgent.TargetPosition = value;
		}
	}
	
	public static int Height { get; set; } = 100;
	public static int Width { get; set; } = 100;
	
	public long Id { get; set; }
	
	public Texture2D IconImage
	{
		get => icon.Texture;
		set => icon.Texture = value;
	}
	
	public bool Selected => selectionSprite.Visible;
	
	[Export]
	public bool SnapToGrid { get; set; } = true;
	
	[Export]
	private float speed = 500.0f;
	
	private Sprite2D icon;
	private NavigationAgent2D navAgent;
	private Sprite2D selectionSprite;
	
	public override void _InputEvent(Viewport viewport, InputEvent evt, int shapeIdx)
	{
		if(evt.IsActionPressed(Actions.Select))
			toggleSelection();
	}
	
	public override void _PhysicsProcess(double delta)
	{
		if(!navAgent.IsNavigationFinished())
		{
			Velocity = GlobalPosition.DirectionTo(navAgent.GetNextPathPosition()) * speed;
			MoveAndSlide();
		}
	}
	
	public override void _Ready()
	{
		SetPhysicsProcess(false);
		
		icon = GetNode<Sprite2D>(NodePaths.Icon);
		navAgent = GetNode<NavigationAgent2D>(NodePaths.NavigationAgent);
		selectionSprite = GetNode<Sprite2D>(NodePaths.Selection);
		
		Callable.From(prepareNavigationAgent).CallDeferred();
	}
	
	private void handleNavigationFinished()
	{
		GlobalPosition = Destination;
		EmitSignal(SignalName.ReachedDestination, Id, GlobalPosition);
	}
	
	private async void prepareNavigationAgent()
	{
		await ToSignal(GetTree(), SceneTree.SignalName.PhysicsFrame);
		navAgent.TargetPosition = GlobalPosition;
		SetPhysicsProcess(true);
		navAgent.NavigationFinished += handleNavigationFinished;
	}
	
	private void toggleSelection()
	{
		if (selectionSprite.Visible)
		{
			selectionSprite.Hide();
			EmitSignal(SignalName.TokenDeselected, this);
		}
		else
		{
			selectionSprite.Show();
			EmitSignal(SignalName.TokenSelected, this);
		}
	}
}
