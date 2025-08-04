using Godot;

namespace Vtt.Board;

public partial class CommandCard : PanelContainer
{
	[Signal]
	public delegate void MovePressedEventHandler();
	
	private Button moveButton;
	
	public override void _Ready()
	{
		moveButton = GetNode<Button>("%Move");
		moveButton.Pressed += handleMovePress;
	}
	
	private void handleMovePress() => EmitSignal(SignalName.MovePressed);
}
