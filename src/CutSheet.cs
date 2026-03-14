using System.Collections.Generic;

internal class CutSheet
{
	public List<CutLine> Lines;

	public Remain remain;

	public int Sheet_Index;

	public int nList;

	public long PartsSq;

	public List<Part> parts;

	public bool isFull;

	public int ALG;

	public void AddLine(CutLine Line, int Blade)
	{
		if (remain.L == Line.L && remain.W != Line.W)
		{
			remain.W = remain.W - Blade - Line.W;
			remain.Y = remain.Y + Blade + Line.W;
		}
		else if (remain.L != Line.L && remain.W == Line.W)
		{
			remain.L = remain.L - Blade - Line.L;
			remain.X = remain.X + Blade + Line.L;
		}
		else if (remain.L == Line.L && remain.W == Line.W)
		{
			remain.L = 0;
			remain.W = 0;
			remain.X = remain.X + Blade + Line.L;
			remain.Y = remain.Y + Blade + Line.W;
		}
		if (remain.L < 0)
		{
			remain.L = 0;
		}
		if (remain.W < 0)
		{
			remain.W = 0;
		}
		remain.Sq = remain.L * remain.W;
		PartsSq += Line.PartsSq;
		for (int i = 0; i < Line.lineParts.Count; i++)
		{
			for (int j = 0; j < parts.Count; j++)
			{
				if (parts[j].Npart == Line.lineParts[i].Npart)
				{
					parts[j].nPlased++;
					j = parts.Count;
				}
			}
		}
		Lines.Add(Line);
	}
}
