using System.Linq;
using Godot;
using Vtt.Board;

namespace Vtt;

public partial class OfflineBoard : Node2D
{
	private static class NodePaths
	{
		public static readonly NodePath Background = new("%Background");
		public static readonly NodePath CommandCard = new("%CommandCard");
		public static readonly NodePath Grid = new("%Grid");
		public static readonly NodePath Line = new("%Line");
		public static readonly NodePath TokenList = new("%TokenList");
	}
	
	private MeshInstance2D background;
	private Node2D grid;
	private Node2D tokenList;
	
	private Vector2 clickedPosition;
	
	private CommandCard commandCard;
	private Line2D line;
	
	private bool moveMode;
	private bool tokenMoving;

	public override void _ExitTree()
	{
		if(IsInstanceValid(commandCard))
			commandCard.MovePressed -= handleMovePressed;
		
		foreach(var tile in grid.GetChildren().Cast<GridTile>())
			tile.AnnounceGridTileCenter -= handleGridAnnounce;
		
		foreach(var token in tokenList.GetChildren().Cast<Token>())
		{
			token.ReachedDestination -= handleTokenPositionUpdated;
			token.TokenDeselected -= handleTokenDeselected;
			token.TokenSelected -= handleTokenSelected;
		}
		
		base._ExitTree();
	}
	
	public override void _Ready()
	{
		commandCard = GetNode<CommandCard>(NodePaths.CommandCard);
		background = GetNode<MeshInstance2D>(NodePaths.Background);
		grid = GetNode<Node2D>(NodePaths.Grid);
		line = GetNode<Line2D>(NodePaths.Line);
		tokenList = GetNode<Node2D>(NodePaths.TokenList);
		
		var bgSize = background.Mesh.GetAabb().Size;
		drawGrid((long)bgSize.Y, (long)bgSize.X);
		
		addToken(new(25f, 25f));
	}
	
	public override void _UnhandledInput(InputEvent evt)
	{
		if(evt is InputEventMouseButton iemb)
		{
			handleEventMovePressed(ref iemb);
			handleEventSelectPressed(ref iemb);
		}
	}
	
	private void addToken(Vector2 position = default)
	{
		var tokenScene = GD.Load<PackedScene>(Scenes.Token);
		if(tokenScene.CanInstantiate())
		{
			var token = tokenScene.Instantiate<Token>();
			tokenList.AddChild(token);
			token.Id = tokenList.GetChildCount();
			token.Position = position;
			token.ReachedDestination += handleTokenPositionUpdated;
			token.TokenDeselected += handleTokenDeselected;
			token.TokenSelected += handleTokenSelected;
		}
	}
	
	private void drawGrid(long height, long width)
	{
		var scene = GD.Load<PackedScene>(Scenes.GridTile);
		if(scene.CanInstantiate())
		{
			var rows = (height / Token.Height) + (height % Token.Height > 0 ? 1 : 0);
			var columns = (width / Token.Width) + (width % Token.Width > 0 ? 1 : 0);
			
			var y = -(height / 2) + (Token.Height / 2);
			for (var r = 0; r < rows; r++)
			{
				var x = -(width / 2) + (Token.Width / 2);
				for(var c = 0; c < columns; c++)
				{
					var tile = scene.Instantiate<GridTile>();
					grid.AddChild(tile);
					tile.Position = new(x, y);
					tile.AnnounceGridTileCenter += handleGridAnnounce;
					
					x += Token.Width;
				}
				
				y += Token.Height;
			}
		}
	}
	
	private void handleEventMovePressed(ref InputEventMouseButton evt)
	{
		if(!tokenMoving && moveMode && evt.IsActionPressed(Actions.Move))
		{
			foreach(var token in tokenList.GetChildren()
				.Cast<Token>()
				.Where(t => t.Selected))
			{
				token.Destination = token.SnapToGrid
					? clickedPosition
					: evt.GlobalPosition;
				
				tokenMoving = true;
			}
		}
	}
	
	private void handleEventSelectPressed(ref InputEventMouseButton evt)
	{
		if(!tokenMoving && evt.IsActionPressed(Actions.Select))
		{
			line.ClearPoints();
			
			var token = tokenList.GetChildren()
				.Cast<Token>()
				.Where(t => t.Selected)
				.FirstOrDefault();
			
			if(token is not null)
			{
				line.AddPoint(token.GlobalPosition);
				line.AddPoint(clickedPosition);
			}
			
			if(!line.Points.IsEmpty())
				line.Show();
		}
		else
		{
			if(line.Visible)
				line.Hide();
		}
	}
	
	private void handleGridAnnounce(Vector2 center) => clickedPosition = center;
	private void handleMovePressed() => moveMode = !moveMode;
	private void handleTokenPositionUpdated(long id, Vector2 position) => tokenMoving = false;
	private void handleTokenDeselected(Token token) => commandCard.Hide();
	private void handleTokenSelected(Token token) => commandCard.Show();
}
