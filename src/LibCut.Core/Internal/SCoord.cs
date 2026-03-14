public struct SCoord
{
	public int list;

	public int X;

	public int Y;

	public bool Used;

	public bool Drawed;

	public SCoord(int l, int x, int y, bool used)
	{
		list = l;
		X = x;
		Y = y;
		Used = used;
		Drawed = false;
	}
}
