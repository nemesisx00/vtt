using Godot;

namespace Vtt.Extensions;

public static class MathExtensions
{
	public static Vector2 Clamp(this Vector2 instance, float min, float max)
	{
		Vector2 vmax = new(max, max);
		Vector2 vmin = new(min, min);
		return instance.Clamp(vmin, vmax);
	}
}
