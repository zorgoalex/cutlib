using System.Collections.Generic;

public static class AlgUtils
{
	public static List<CPart> ConvertParts_to_CParts(List<Part> P)
	{
		List<CPart> list = new List<CPart>();
		for (int i = 0; i < P.Count; i++)
		{
			if (P[i].nPlased < P[i].Amount)
			{
				CPart cPart = new CPart();
				cPart.L = P[i].Length_mm * 10;
				cPart.W = P[i].Width_mm * 10;
				cPart.Qty = P[i].Amount - P[i].nPlased;
				cPart.iD_in_Order = i;
				cPart.Turn = P[i].Turn;
				cPart.Plased = 0;
				list.Add(cPart);
			}
		}
		return list;
	}

	public static List<CPart> Copy_CParts(List<CPart> P)
	{
		List<CPart> list = new List<CPart>();
		for (int i = 0; i < P.Count; i++)
		{
			CPart cPart = new CPart();
			cPart.L = P[i].L;
			cPart.W = P[i].W;
			cPart.Qty = P[i].Qty;
			cPart.iD_in_Order = P[i].iD_in_Order;
			cPart.Turn = P[i].Turn;
			cPart.Plased = P[i].Plased;
			list.Add(cPart);
		}
		return list;
	}

	public static int GetNextRemain(Order o)
	{
		int result = -1;
		long num = 100000000L;
		for (int i = 0; i < o.Snips.Count; i++)
		{
			if (o.Snips[i].nCutted < o.Snips[i].Amount && num >= o.Snips[i].Sq)
			{
				int num2 = o.Snips[i].Length_mm;
				int num3 = o.Snips[i].Width_mm;
				if (o.Snips[i].offcut)
				{
					num2 -= o.parameters.Padding;
					num3 -= o.parameters.Padding;
				}
				if (FastFindFirst_Part(o.Parts, num2, num3))
				{
					num = o.Snips[i].Sq;
					result = i;
				}
			}
		}
		return result;
	}

	public static bool FastFindFirst_CPart(List<CPart> parts, int LO, int WO)
	{
		bool result = false;
		if (LO > 0 && WO > 0)
		{
			for (int num = parts.Count - 1; num >= 0; num--)
			{
				if (parts[num].Plased < parts[num].Qty && ((LO >= parts[num].L && WO >= parts[num].W) || (parts[num].Turn && LO >= parts[num].W && WO >= parts[num].L)))
				{
					result = true;
					num = -1;
				}
			}
		}
		return result;
	}

	public static bool FastFindFirst_Part(List<Part> parts, int LO, int WO)
	{
		bool result = false;
		if (LO > 0 && WO > 0)
		{
			for (int i = 0; i < parts.Count; i++)
			{
				if (parts[i].nPlased < parts[i].Amount && (((LO - parts[i].Length_mm) * 100 >= 0 && (WO - parts[i].Width_mm) * 100 >= 0) || (parts[i].Turn && (LO - parts[i].Width_mm) * 100 >= 0 && (WO - parts[i].Length_mm) * 100 >= 0)))
				{
					result = true;
					i = parts.Count;
				}
			}
		}
		return result;
	}

	public static void Write_Sheets_to_Order(Order o, List<CSheet> Ss)
	{
		int num = o.parameters.Blade * 10;
		int num2 = o.parameters.Padding * 10;
		for (int i = 0; i < Ss.Count; i++)
		{
			switch (Ss[i].Alg)
			{
			case 3:
			{
				for (int num10 = 0; num10 < Ss[i].Lines.Count; num10++)
				{
					for (int num11 = 0; num11 < Ss[i].Lines[num10].PartIDs.Count; num11++)
					{
						bool isTurn3 = false;
						if (Ss[i].Lines[num10].PartIDs[num11] < -1)
						{
							_ = Ss[i].Lines[num10].PartIDs[num11];
							isTurn3 = true;
						}
						else
						{
							_ = Ss[i].Lines[num10].PartIDs[num11];
						}
						Crd crd3 = Ss[i].Lines[num10].Parts_Crds[num11];
						Write_Part(o.Parts[crd3.id_in_order], Ss[i].Lines[num10].crd.X + crd3.X, Ss[i].Lines[num10].crd.Y + crd3.Y, i + 1, -1, isTurn3);
						o.PartsPlased++;
					}
					for (int num12 = 0; num12 < Ss[i].Lines[num10].Snips.Count; num12++)
					{
						CSnip cSnip3 = Ss[i].Lines[num10].Snips[num12];
						o.NSnips.Add(Write_NSnip(cSnip3.L, cSnip3.W, Ss[i].Lines[num10].crd.X + cSnip3.CRD.X, Ss[i].Lines[num10].crd.Y + cSnip3.CRD.Y, i + 1, -1));
					}
				}
				CSnip remain3 = Ss[i].Remain;
				o.NSnips.Add(Write_NSnip(remain3.L, remain3.W, remain3.CRD.X, remain3.CRD.Y, i + 1, -1));
				o.SheetCount++;
				break;
			}
			case 1:
			{
				int num6 = num2;
				int num7 = num2;
				while (Ss[i].Lines.Count > 0)
				{
					int num8 = 0;
					int index2 = -1;
					for (int m = 0; m < Ss[i].Lines.Count; m++)
					{
						if ((Ss[i].Lines[m].W - num8) * 100 > 0 || ((Ss[i].Lines[m].W - num8) * 100 == 0 && (int)((Ss[i].Lines[m].Parts_Sq - Ss[i].Lines[index2].Parts_Sq) * 100.0) > 0))
						{
							index2 = m;
							num8 = Ss[i].Lines[m].W;
						}
					}
					for (int n = 0; n < Ss[i].Lines[index2].PartIDs.Count; n++)
					{
						bool isTurn2 = false;
						if (Ss[i].Lines[index2].PartIDs[n] < -1)
						{
							_ = Ss[i].Lines[index2].PartIDs[n];
							isTurn2 = true;
						}
						else
						{
							_ = Ss[i].Lines[index2].PartIDs[n];
						}
						Crd crd2 = Ss[i].Lines[index2].Parts_Crds[n];
						Write_Part(o.Parts[crd2.id_in_order], num6 + crd2.X, num7 + crd2.Y, i + 1, -1, isTurn2);
						o.PartsPlased++;
					}
					for (int num9 = 0; num9 < Ss[i].Lines[index2].Snips.Count; num9++)
					{
						CSnip cSnip2 = Ss[i].Lines[index2].Snips[num9];
						o.NSnips.Add(Write_NSnip(cSnip2.L, cSnip2.W, num6 + cSnip2.CRD.X, num7 + cSnip2.CRD.Y, i + 1, -1));
					}
					num7 = num7 + num + Ss[i].Lines[index2].W;
					Ss[i].Lines.RemoveAt(index2);
				}
				CSnip remain2 = Ss[i].Remain;
				o.NSnips.Add(Write_NSnip(remain2.L, remain2.W, num6, num7, i + 1, -1));
				o.SheetCount++;
				break;
			}
			case 2:
			{
				int num3 = num2;
				int num4 = num2;
				while (Ss[i].Lines.Count > 0)
				{
					int num5 = 0;
					int index = -1;
					for (int j = 0; j < Ss[i].Lines.Count; j++)
					{
						if ((Ss[i].Lines[j].L - num5) * 100 > 0 || ((Ss[i].Lines[j].L - num5) * 100 == 0 && (int)((Ss[i].Lines[j].Parts_Sq - Ss[i].Lines[index].Parts_Sq) * 100.0) > 0))
						{
							index = j;
							num5 = Ss[i].Lines[j].L;
						}
					}
					for (int k = 0; k < Ss[i].Lines[index].PartIDs.Count; k++)
					{
						bool isTurn = false;
						if (Ss[i].Lines[index].PartIDs[k] < -1)
						{
							_ = Ss[i].Lines[index].PartIDs[k];
							isTurn = true;
						}
						else
						{
							_ = Ss[i].Lines[index].PartIDs[k];
						}
						Crd crd = Ss[i].Lines[index].Parts_Crds[k];
						Write_Part(o.Parts[crd.id_in_order], num3 + crd.X, num4 + crd.Y, i + 1, -1, isTurn);
						o.PartsPlased++;
					}
					for (int l = 0; l < Ss[i].Lines[index].Snips.Count; l++)
					{
						CSnip cSnip = Ss[i].Lines[index].Snips[l];
						o.NSnips.Add(Write_NSnip(cSnip.L, cSnip.W, num3 + cSnip.CRD.X, num4 + cSnip.CRD.Y, i + 1, -1));
					}
					num3 = num3 + num + Ss[i].Lines[index].L;
					Ss[i].Lines.RemoveAt(index);
				}
				CSnip remain = Ss[i].Remain;
				o.NSnips.Add(Write_NSnip(remain.L, remain.W, num3, num4, i + 1, -1));
				o.SheetCount++;
				break;
			}
			}
		}
	}

	public static void Write_Snip_to_Order(Order o, int i_S, CSheet sheet)
	{
		int num = o.parameters.Blade * 10;
		int num2 = o.parameters.Padding * 10;
		int list = i_S * -1 - 2;
		int nlist = o.Snips[i_S].nCutted + 1;
		switch (sheet.Alg)
		{
		case 3:
		{
			for (int num9 = 0; num9 < sheet.Lines.Count; num9++)
			{
				for (int num10 = 0; num10 < sheet.Lines[num9].PartIDs.Count; num10++)
				{
					bool isTurn3 = false;
					if (sheet.Lines[num9].PartIDs[num10] < -1)
					{
						_ = sheet.Lines[num9].PartIDs[num10];
						isTurn3 = true;
					}
					else
					{
						_ = sheet.Lines[num9].PartIDs[num10];
					}
					Crd crd3 = sheet.Lines[num9].Parts_Crds[num10];
					Write_Part(o.Parts[crd3.id_in_order], sheet.Lines[num9].crd.X + crd3.X, sheet.Lines[num9].crd.Y + crd3.Y, list, nlist, isTurn3);
					o.PartsPlased++;
				}
				for (int num11 = 0; num11 < sheet.Lines[num9].Snips.Count; num11++)
				{
					CSnip cSnip3 = sheet.Lines[num9].Snips[num11];
					o.NSnips.Add(Write_NSnip(cSnip3.L, cSnip3.W, sheet.Lines[num9].crd.X + cSnip3.CRD.X, sheet.Lines[num9].crd.Y + cSnip3.CRD.Y, list, nlist));
				}
			}
			CSnip remain3 = sheet.Remain;
			o.NSnips.Add(Write_NSnip(remain3.L, remain3.W, remain3.CRD.X, remain3.CRD.Y, list, nlist));
			o.Snips[i_S].nCutted++;
			o.UsedSnipsCount++;
			break;
		}
		case 1:
		{
			int num6 = num2;
			int num7 = num2;
			while (sheet.Lines.Count > 0)
			{
				int num8 = 0;
				int index2 = -1;
				for (int l = 0; l < sheet.Lines.Count; l++)
				{
					if ((sheet.Lines[l].W - num8) * 100 > 0 || ((sheet.Lines[l].W - num8) * 100 == 0 && (int)((sheet.Lines[l].Parts_Sq - sheet.Lines[index2].Parts_Sq) * 100.0) > 0))
					{
						index2 = l;
						num8 = sheet.Lines[l].W;
					}
				}
				for (int m = 0; m < sheet.Lines[index2].PartIDs.Count; m++)
				{
					bool isTurn2 = false;
					if (sheet.Lines[index2].PartIDs[m] < -1)
					{
						_ = sheet.Lines[index2].PartIDs[m];
						isTurn2 = true;
					}
					else
					{
						_ = sheet.Lines[index2].PartIDs[m];
					}
					Crd crd2 = sheet.Lines[index2].Parts_Crds[m];
					Write_Part(o.Parts[crd2.id_in_order], num6 + crd2.X, num7 + crd2.Y, list, nlist, isTurn2);
					o.PartsPlased++;
				}
				for (int n = 0; n < sheet.Lines[index2].Snips.Count; n++)
				{
					CSnip cSnip2 = sheet.Lines[index2].Snips[n];
					o.NSnips.Add(Write_NSnip(cSnip2.L, cSnip2.W, num6 + cSnip2.CRD.X, num7 + cSnip2.CRD.Y, list, nlist));
				}
				num7 = num7 + num + sheet.Lines[index2].W;
				sheet.Lines.RemoveAt(index2);
			}
			CSnip remain2 = sheet.Remain;
			o.NSnips.Add(Write_NSnip(remain2.L, remain2.W, num6, num7, list, nlist));
			o.Snips[i_S].nCutted++;
			o.UsedSnipsCount++;
			break;
		}
		case 2:
		{
			int num3 = num2;
			int num4 = num2;
			while (sheet.Lines.Count > 0)
			{
				int num5 = 0;
				int index = -1;
				for (int i = 0; i < sheet.Lines.Count; i++)
				{
					if ((sheet.Lines[i].L - num5) * 100 > 0 || ((sheet.Lines[i].L - num5) * 100 == 0 && (int)((sheet.Lines[i].Parts_Sq - sheet.Lines[index].Parts_Sq) * 100.0) > 0))
					{
						index = i;
						num5 = sheet.Lines[i].L;
					}
				}
				for (int j = 0; j < sheet.Lines[index].PartIDs.Count; j++)
				{
					bool isTurn = false;
					if (sheet.Lines[index].PartIDs[j] < -1)
					{
						_ = sheet.Lines[index].PartIDs[j];
						isTurn = true;
					}
					else
					{
						_ = sheet.Lines[index].PartIDs[j];
					}
					Crd crd = sheet.Lines[index].Parts_Crds[j];
					Write_Part(o.Parts[crd.id_in_order], num3 + crd.X, num4 + crd.Y, list, nlist, isTurn);
					o.PartsPlased++;
				}
				for (int k = 0; k < sheet.Lines[index].Snips.Count; k++)
				{
					CSnip cSnip = sheet.Lines[index].Snips[k];
					o.NSnips.Add(Write_NSnip(cSnip.L, cSnip.W, num3 + cSnip.CRD.X, num4 + cSnip.CRD.Y, list, nlist));
				}
				num3 = num3 + num + sheet.Lines[index].L;
				sheet.Lines.RemoveAt(index);
			}
			CSnip remain = sheet.Remain;
			o.NSnips.Add(Write_NSnip(remain.L, remain.W, num3, num4, list, nlist));
			o.Snips[i_S].nCutted++;
			o.UsedSnipsCount++;
			break;
		}
		}
	}

	private static void Write_Part(Part part, int X, int Y, int list, int nlist, bool isTurn)
	{
		int nPlased = part.nPlased;
		part.Coords[nPlased].X = X / 10;
		part.Coords[nPlased].Y = Y / 10;
		part.Coords[nPlased].isTurn = isTurn;
		part.Coords[nPlased].list = list;
		part.Coords[nPlased].nlist = nlist;
		part.Coords[nPlased].Cutted = true;
		part.Coords[nPlased].onList = true;
		part.nPlased++;
	}

	private static Snip Write_NSnip(int L, int W, int X, int Y, int list, int nlist)
	{
		Snip snip = new Snip();
		snip.Length_mm = L / 10;
		snip.Width_mm = W / 10;
		snip.onList = true;
		snip.Sq = snip.Length_mm * snip.Width_mm;
		snip.list = list;
		snip.nlist = nlist;
		snip.Amount = 1;
		snip.X = X / 10;
		snip.Y = Y / 10;
		return snip;
	}
}
