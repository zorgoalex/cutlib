using System.Collections.Generic;

public class Snip
{
	public string _length = "";
	public string _width = "";
	public int Length_mm;
	public int Width_mm;
	public int Amount = 1;
	public bool offcut;
	public long Sq;
	public int X = -1;
	public int Y = -1;
	public int list = -1;
	public int nlist = -1;
	public bool onList;
	public bool full;
	public List<SCoord> SCoords = new List<SCoord>();
	public int nCutted;
	public int nDrawed;
}
