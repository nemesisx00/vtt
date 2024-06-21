namespace Vtt.Network;

public struct ClientStatus()
{
	public long id = -1;
	public bool connected = default;
	public bool loggedIn = default;
	public string username = default;
	
	public void Disconnect()
	{
		id = -1;
		connected = false;
		loggedIn = false;
		username = string.Empty;
	}
}
