using Godot;

namespace Vtt;

public partial class Gameplay : Node2D
{	private sealed class NodePaths
	{
		public static readonly NodePath GameplayUI = new("%GameplayUI");
	}
	
	private VBoxContainer gameplayUi;
	
	public override void _Ready()
	{
		gameplayUi = GetNode<VBoxContainer>(NodePaths.GameplayUI);
	}
}
