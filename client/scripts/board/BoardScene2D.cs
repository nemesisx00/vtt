using Godot;
using Godot.Collections;

namespace Vtt.Board;

public partial class BoardScene2D : Node2D
{
	private static class NodePaths
	{
		public static readonly NodePath Background = new("%Background");
		public static readonly NodePath NavigationRegion = new("%NavigationRegion2D");
		public static readonly NodePath TokenList = new("%Tokens");
	}
	
	public Texture2D BackgroundTexture
	{
		get => background.Texture;
		set => background.Texture = value;
	}
	
	public Array<int[]> Polygons
	{
		get => navRegion.NavigationPolygon.Polygons;
		
		set
		{
			navRegion.NavigationPolygon.Polygons = value;
			navRegion.BakeNavigationPolygon();
		}
	}
	
	private Sprite2D background;
	private NavigationRegion2D navRegion;
	private Node tokenList;
	
	public override void _Ready()
	{
		background = GetNode<Sprite2D>(NodePaths.Background);
		navRegion = GetNode<NavigationRegion2D>(NodePaths.NavigationRegion);
		tokenList = GetNode(NodePaths.TokenList);
	}
}
