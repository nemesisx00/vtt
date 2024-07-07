using Godot;
using Godot.Collections;
using System.Collections.Generic;
using System.Linq;

namespace Vtt.Board;

public partial class BoardScene2D : Node2D
{
	private static class NodePaths
	{
		public static readonly NodePath Background = new("%Background");
		public static readonly NodePath Grid = new("%Grid");
		public static readonly NodePath NavigationRegion = new("%NavigationRegion2D");
		public static readonly NodePath TokenList = new("%Tokens");
	}
	
	public Texture2D BackgroundTexture
	{
		get => background.Texture;
		set => background.Texture = value;
	}
	
	//TODO: Enable loading custom navigation polygons for the scene from the backend
	/*
		This will require modifying the grid tile scene to not include a
		navigation region or making the GenerateGrid method use a more complex
		algorithm to ensure the grid only exists within the navigable space
		defined by this array of polygons.
	*/
	/*
	public Array<int[]> Polygons
	{
		get => navRegion.NavigationPolygon.Polygons;
		
		set
		{
			navRegion.NavigationPolygon.Polygons = value;
			navRegion.BakeNavigationPolygon();
		}
	}
	*/
	
	public List<Token> Tokens => tokenList.GetChildren()
		.Cast<Token>()
		.ToList();
	
	private Sprite2D background;
	private Node2D grid;
	//private NavigationRegion2D navRegion;
	private Node2D tokenList;
	
	public override void _Ready()
	{
		background = GetNode<Sprite2D>(NodePaths.Background);
		grid = GetNode<Node2D>(NodePaths.Grid);
		//navRegion = GetNode<NavigationRegion2D>(NodePaths.NavigationRegion);
		tokenList = GetNode<Node2D>(NodePaths.TokenList);
	}
	
	public void AddToken(float x, float y) => AddToken(new(x, y));
	public void AddToken(Vector2 position = default)
	{
		var tokenScene = GD.Load<PackedScene>(Scenes.Token);
		if(tokenScene.CanInstantiate())
		{
			var token = tokenScene.Instantiate<Token>();
			tokenList.AddChild(token);
			token.Position = position;
		}
	}
	
	public void GenerateGrid(Vector2 size) => GenerateGrid((long)size.Y, (long)size.X);
	public void GenerateGrid(long height, long width)
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
					var tile = scene.Instantiate<Node2D>();
					grid.AddChild(tile);
					tile.Position = new(x, y);
					
					x += Token.Width;
				}
				
				y += Token.Height;
			}
		}
	}
}
