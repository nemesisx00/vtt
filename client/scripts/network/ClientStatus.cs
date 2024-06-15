namespace Vtt.Network;

public struct ClientStatus()
{
	public long id = -1;
	public bool connected;
	public bool loggedIn;
	
	public void Disconnect()
	{
		id = -1;
		connected = false;
		loggedIn = false;
	}
}
