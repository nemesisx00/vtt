using System.Collections.Generic;

namespace Vtt.Network.Payload;

public struct Scene2DData(long height, long width, byte[] background)
{
	public static Scene2DData Deserialize(Dictionary<string, string> data, Dictionary<string, byte[]> binaryData)
		=> new(
			long.TryParse(data["height"], out long height)
				? height
				: -1,
			long.TryParse(data["width"], out long width)
				? width
				: -1,
			binaryData["background"]
		);
	
	public readonly byte[] Background => background;
	public readonly long BackgroundHeight => height;
	public readonly long BackgroundWidth => width;
}
