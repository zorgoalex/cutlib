public class Coord
{
	public int list = -1;

	public int nlist = -1;

	public int X = -1;

	public int Y = -1;

	public bool onList;

	public bool isTurn;

	public bool Cutted;

	public Coord()
	{
	}

	public Coord(int _list, int _nlist, int x, int y, bool onlist, bool isturn, bool cutted)
	{
		list = _list;
		nlist = _nlist;
		X = x;
		Y = y;
		onList = onlist;
		isTurn = isturn;
		Cutted = cutted;
	}
}
