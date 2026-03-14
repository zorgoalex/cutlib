using System;
using System.Collections.Generic;

public class Length_Alg
{
	public int THE_SAME_PARTS_LIMIT = 25;

	public int LINES_LIMIT = 200;

	public int LINES_SORT_ITERS_LIMIT = 4;

	public int PARTS_SORT_LIMIT = 2;

	public double TIME_GET_LINES_LIMIT = 2.0;

	private List<CPart> CPARTS;

	private List<CSheet> SHEETS;

	private int P;

	private int B;

	private int L_L;

	private int L_W;

	private int minL;

	private int minW;

	public CutTimes CT;

	public void StartCutting(Order order)
	{
		_ = DateTime.Now.TimeOfDay.TotalMilliseconds;
		Utils.ClearCuttingInfo(order);
		SHEETS = new List<CSheet>();
		bool flag = false;
		int num = 0;
		while (!flag)
		{
			num++;
			if (FastFindFirstPart(CPARTS, L_L - 2 * P, L_W - 2 * P))
			{
				CSheet cSheet = GetCSheet_LENGTH_CUT(CPARTS, L_L, L_W, B, P, DoublePadding: true, Opti_ON: true, CleanParts: false);
				for (int i = 0; i < cSheet.Lines.Count; i++)
				{
					SET_OFF_Parts_in_Line(CPARTS, cSheet.Lines[i]);
				}
				CSheet cSheet_LENGTH_CUT = GetCSheet_LENGTH_CUT(CPARTS, L_L, L_W, B, P, DoublePadding: true, Opti_ON: false, CleanParts: false);
				if ((int)(cSheet_LENGTH_CUT.Parts_Sq - cSheet.Parts_Sq) > 0)
				{
					cSheet = cSheet_LENGTH_CUT;
				}
				else
				{
					for (int j = 0; j < cSheet_LENGTH_CUT.Lines.Count; j++)
					{
						SET_OFF_Parts_in_Line(CPARTS, cSheet_LENGTH_CUT.Lines[j]);
					}
					for (int k = 0; k < cSheet.Lines.Count; k++)
					{
						SET_ON_Parts_in_Line(CPARTS, cSheet.Lines[k]);
					}
				}
				SHEETS.Add(cSheet);
			}
			else
			{
				flag = true;
			}
		}
		Write_Sheets_to_Order_LENGTH_CUT(order, SHEETS);
	}

	private List<CPart> Clean_CParts(List<CPart> P)
	{
		for (int i = 0; i < P.Count; i++)
		{
			if (P[i].Qty > P[i].Plased && P[i].Plased != 0)
			{
				P[i].Qty = P[i].Qty - P[i].Plased;
				P[i].Plased = 0;
			}
			else if (P[i].Qty == P[i].Plased)
			{
				P.RemoveAt(i);
				i--;
			}
		}
		return P;
	}

	public CSheet GetCSheet_LENGTH_CUT(List<CPart> parts, int ListLength, int ListWidth, int Blade, int Padding, bool DoublePadding, bool Opti_ON, bool CleanParts)
	{
		CT = new CutTimes();
		double totalMilliseconds = DateTime.Now.TimeOfDay.TotalMilliseconds;
		L_L = ListLength;
		L_W = ListWidth;
		P = Padding;
		B = Blade;
		if (CleanParts)
		{
			parts = Clean_CParts(parts);
		}
		CSheet cSheet = new CSheet();
		cSheet.Alg = 1;
		cSheet.Lines = new List<CLine>();
		cSheet.Lines_index = new List<int>();
		cSheet.L = L_L;
		cSheet.W = L_W;
		int num = P;
		if (DoublePadding)
		{
			num *= 2;
		}
		int num2 = cSheet.L - num;
		int num3 = cSheet.W - num;
		List<CLine> cLines_LENGTH_CUT = GetCLines_LENGTH_CUT(parts, num2, num3, Opti_ON);
		int num4 = num3;
		int num5 = 0;
		for (int i = 0; i < cLines_LENGTH_CUT.Count; i++)
		{
			if (num4 > cLines_LENGTH_CUT[i].W)
			{
				num4 = cLines_LENGTH_CUT[i].W;
			}
			num5 += cLines_LENGTH_CUT[i].W;
		}
		int num6 = num3;
		for (int j = 0; j < cLines_LENGTH_CUT.Count; j++)
		{
			if (num6 >= cLines_LENGTH_CUT[j].W)
			{
				num6 = num6 - cLines_LENGTH_CUT[j].W - B;
				cSheet.Lines.Add(cLines_LENGTH_CUT[j]);
				cSheet.Lines_index.Add(j);
				cLines_LENGTH_CUT[j].onSheet = true;
				if (num4 >= num6)
				{
					j = cLines_LENGTH_CUT.Count;
				}
			}
		}
		bool flag = false;
		int num7 = 0;
		while (!flag && num7 < LINES_SORT_ITERS_LIMIT)
		{
			num7++;
			bool check = false;
			int num8 = -1;
			int num9 = -1;
			int[] array = null;
			double num10 = 0.0;
			for (int k = 0; k < cSheet.Lines.Count - 1; k++)
			{
				for (int l = k + 1; l < cSheet.Lines.Count; l++)
				{
					cSheet.Lines[k].onSheet = false;
					cSheet.Lines[l].onSheet = false;
					int wO = num6 + B + cSheet.Lines[k].W + B + cSheet.Lines[l].W;
					int[] array2 = Find_Zamena_Lines_LENGTH_CUT(cLines_LENGTH_CUT, wO, num4, out check);
					if (cSheet.Lines_index[k] != array2[0] || cSheet.Lines_index[l] != array2[1] || array2[2] != -1)
					{
						int num11 = cSheet.Lines[k].W + cSheet.Lines[l].W;
						double num12 = cSheet.Lines[k].Parts_Sq + cSheet.Lines[l].Parts_Sq;
						int num13 = 0;
						double num14 = 0.0;
						for (int m = 0; m < 3; m++)
						{
							if (array2[m] != -1)
							{
								num14 += cLines_LENGTH_CUT[array2[m]].Parts_Sq;
								num13 += cLines_LENGTH_CUT[array2[m]].W;
							}
						}
						if (num13 >= num11 && (int)(num14 - num12) >= 0 && ((int)(num14 - num10) > 0 || ((int)(num14 - num10) == 0 && num13 - cSheet.Lines[num8].W - cSheet.Lines[num9].W > 0)))
						{
							num8 = k;
							num9 = l;
							array = array2;
							num10 = num14;
						}
					}
					cSheet.Lines[k].onSheet = true;
					cSheet.Lines[l].onSheet = true;
				}
			}
			if (num8 != -1 && num9 != -1)
			{
				num6 = num6 + B + cSheet.Lines[num8].W + B + cSheet.Lines[num9].W;
				cSheet.Lines[num8].onSheet = false;
				cSheet.Lines[num9].onSheet = false;
				cSheet.Lines.Remove(cSheet.Lines[num8]);
				cSheet.Lines.Remove(cSheet.Lines[num9 - 1]);
				cSheet.Lines_index.Remove(num8);
				cSheet.Lines_index.Remove(num9 - 1);
				for (int n = 0; n < 3; n++)
				{
					if (array[n] != -1)
					{
						cSheet.Lines.Add(cLines_LENGTH_CUT[array[n]]);
						cSheet.Lines_index.Add(array[n]);
						cLines_LENGTH_CUT[array[n]].onSheet = true;
						num6 = num6 - B - cLines_LENGTH_CUT[array[n]].W;
					}
				}
			}
			else
			{
				flag = true;
			}
		}
		cSheet.Remain = new CSnip();
		cSheet.Remain.L = num2;
		cSheet.Remain.W = num6;
		for (int num15 = 0; num15 < cSheet.Lines.Count - 1; num15++)
		{
			for (int num16 = num15 + 1; num16 < cSheet.Lines.Count; num16++)
			{
				if (cSheet.Lines[num16].W > cSheet.Lines[num15].W)
				{
					int value = cSheet.Lines_index[num15];
					cSheet.Lines_index[num15] = cSheet.Lines_index[num16];
					cSheet.Lines_index[num16] = value;
					CLine value2 = cSheet.Lines[num15];
					cSheet.Lines[num15] = cSheet.Lines[num16];
					cSheet.Lines[num16] = value2;
				}
			}
		}
		for (int num17 = 0; num17 < cLines_LENGTH_CUT.Count; num17++)
		{
			if (!cLines_LENGTH_CUT[num17].onSheet)
			{
				SET_OFF_Parts_in_Line(parts, cLines_LENGTH_CUT[num17]);
			}
		}
		cSheet.Parts_Sq = 0.0;
		bool maxWay = true;
		bool optiON = true;
		for (int num18 = cSheet.Lines.Count - 1; num18 >= 0; num18--)
		{
			Continue_Line_LENGTH_CUT(cSheet.Lines[num18], parts, maxWay, optiON);
			cSheet.Parts_Sq += cSheet.Lines[num18].Parts_Sq;
		}
		flag = false;
		while (!flag)
		{
			if (FastFindFirstPart(parts, cSheet.Remain.L, cSheet.Remain.W))
			{
				CLine cLine = new CLine();
				CSnip cSnip = new CSnip();
				cLine.Snips = new List<CSnip>();
				cLine.PartIDs = new List<int>();
				cLine.Parts_Crds = new List<Crd>();
				int num19 = Find_WIDTH_part(parts, cSheet.Remain.L, cSheet.Remain.W, Max_W: true);
				Get_ID_LD_WD(parts, num19, out var _, out var _, out var WD);
				bool rez = false;
				cLine.W = WD;
				cLine.L = cSheet.Remain.L;
				cSnip.CRD = new Crd();
				cSnip.CRD.X = 0;
				cSnip.CRD.Y = 0;
				cSnip.L = cLine.L;
				cSnip.W = cLine.W;
				cLine.Snips.Add(cSnip);
				int io = FindSmallSnip(cLine.Snips, parts);
				Place_Part_to_Line(cLine, parts, num19, io, rez);
				Continue_Line_LENGTH_CUT(cLine, parts, maxWay, OptiON: true);
				cSheet.Remain.W = cSheet.Remain.W - B - cLine.W;
				cSheet.Lines.Add(cLine);
			}
			else
			{
				flag = true;
			}
		}
		CT.CutTime = (DateTime.Now.TimeOfDay.TotalMilliseconds - totalMilliseconds) / 1000.0;
		return cSheet;
	}

	private void Continue_Line_LENGTH_CUT(CLine LINE, List<CPart> parts, bool MaxWay, bool OptiON)
	{
		double totalMilliseconds = DateTime.Now.TimeOfDay.TotalMilliseconds;
		int num = -1;
		for (int num2 = FindSmallSnip(LINE.Snips, parts); num2 >= 0; num2 = FindSmallSnip(LINE.Snips, parts))
		{
			int l = LINE.Snips[num2].L;
			int w = LINE.Snips[num2].W;
			num = ((!MaxWay) ? Find_WIDTH_part(parts, l, w, Max_W: true) : FindMaxSqPart(parts, l, w));
			if (num != -1)
			{
				if (OptiON)
				{
					int[] array = Check_part_for_last_in_Line(parts, l, w, rez: false, num);
					if (num != array[0] && array[0] != -1)
					{
						Place_2_Parts_to_Line(LINE, parts, array, num2);
					}
					else
					{
						Place_Part_to_Line(LINE, parts, num, num2, _rez: false);
					}
				}
				else
				{
					Place_Part_to_Line(LINE, parts, num, num2, _rez: false);
				}
			}
		}
		CT.T_Continue_Line_LENGTH_CUT += (DateTime.Now.TimeOfDay.TotalMilliseconds - totalMilliseconds) / 1000.0;
		CT.C_Continue_Line_LENGTH_CUT++;
	}

	private List<CLine> GetCLines_LENGTH_CUT(List<CPart> parts, int LL, int LW, bool OptiON)
	{
		double totalMilliseconds = DateTime.Now.TimeOfDay.TotalMilliseconds;
		List<CLine> list = new List<CLine>();
		int num = -1;
		int num2 = 0;
		bool flag = false;
		int num3 = LL;
		int num4 = LW;
		while (!flag && num2 < LINES_LIMIT)
		{
			num2++;
			CLine cLine = null;
			CLine PreCut = null;
			CLine cLine2 = null;
			CLine PreCut2 = null;
			CLine cLine3 = null;
			CLine PreCut3 = null;
			CLine cLine4 = null;
			CLine PreCut4 = null;
			num = Find_WIDTH_part(parts, num3, num4, Max_W: false);
			if (num != -1)
			{
				cLine = MakeLine_LENGTH_CUT(parts, num, num3, num4, THE_SAME_MAX: true, MaxWay: false, OptiON, out PreCut);
				SET_ON_Parts_in_Line(parts, PreCut);
				Continue_Line_LENGTH_CUT(PreCut, parts, MaxWay: true, OptiON);
				SET_OFF_Parts_in_Line(parts, PreCut);
				if (cLine != null && PreCut != null && (int)(PreCut.Parts_Sq - cLine.Parts_Sq) > 0)
				{
					cLine = PreCut;
				}
				cLine2 = MakeLine_LENGTH_CUT(parts, num, num3, num4, THE_SAME_MAX: false, MaxWay: false, OptiON, out PreCut2);
				SET_ON_Parts_in_Line(parts, PreCut2);
				Continue_Line_LENGTH_CUT(PreCut2, parts, MaxWay: true, OptiON);
				SET_OFF_Parts_in_Line(parts, PreCut2);
				if (cLine2 != null && PreCut2 != null && (int)(cLine2.Parts_Sq - PreCut2.Parts_Sq) < 0)
				{
					cLine2 = PreCut2;
				}
				if (cLine != null && cLine2 != null)
				{
					if ((int)((cLine.Filling - cLine2.Filling) * 100f) < 0)
					{
						cLine = cLine2;
					}
					else if ((int)((cLine.Filling - cLine2.Filling) * 100f) == 0 && cLine.L < cLine2.L)
					{
						cLine = cLine2;
					}
				}
				else if (cLine == null && cLine2 != null)
				{
					cLine = cLine2;
				}
				int index = num;
				if (num < -1)
				{
					index = num * -1 - 2;
				}
				if (parts[index].Turn)
				{
					if ((num < -1 && num3 >= parts[index].L && num4 >= parts[index].W) || (num > -1 && num3 >= parts[index].W && num4 >= parts[index].L))
					{
						cLine3 = MakeLine_LENGTH_CUT(parts, num * -1 - 2, num3, num4, THE_SAME_MAX: true, MaxWay: false, OptiON, out PreCut3);
						SET_ON_Parts_in_Line(parts, PreCut3);
						Continue_Line_LENGTH_CUT(PreCut3, parts, MaxWay: true, OptiON);
						SET_OFF_Parts_in_Line(parts, PreCut3);
						cLine4 = MakeLine_LENGTH_CUT(parts, num * -1 - 2, num3, num4, THE_SAME_MAX: false, MaxWay: false, OptiON, out PreCut4);
						SET_ON_Parts_in_Line(parts, PreCut4);
						Continue_Line_LENGTH_CUT(PreCut4, parts, MaxWay: true, OptiON);
						SET_OFF_Parts_in_Line(parts, PreCut4);
					}
					if (cLine3 != null && cLine4 != null && PreCut4 != null)
					{
						if ((int)(PreCut3.Parts_Sq - cLine3.Parts_Sq) > 0)
						{
							cLine3 = PreCut3;
						}
						if ((int)(cLine4.Parts_Sq - cLine3.Parts_Sq) > 0)
						{
							cLine3 = cLine4;
						}
						if ((int)(PreCut4.Parts_Sq - cLine3.Parts_Sq) > 0)
						{
							cLine3 = PreCut4;
						}
					}
				}
				if (cLine != null && cLine3 != null && (int)((cLine.Filling - cLine3.Filling) * 100f) < 0)
				{
					cLine = cLine3;
				}
			}
			if (cLine != null)
			{
				SET_ON_Parts_in_Line(parts, cLine);
				list.Add(cLine);
				num4 = num4 - B - cLine.W;
				if (!FastFindFirstPart(parts, num3, num4))
				{
					num3 = LL;
					num4 = LW;
					if (!FastFindFirstPart(parts, num3, num4))
					{
						flag = true;
					}
				}
			}
			else
			{
				flag = true;
			}
			if ((int)(((DateTime.Now.TimeOfDay.TotalMilliseconds - totalMilliseconds) / 1000.0 - TIME_GET_LINES_LIMIT) * 10.0) > 0)
			{
				flag = true;
			}
		}
		CT.T_GetCLines_LENGTH_CUT += (DateTime.Now.TimeOfDay.TotalMilliseconds - totalMilliseconds) / 1000.0;
		CT.C_GetCLines_LENGTH_CUT++;
		return list;
	}

	private CLine MakeLine_LENGTH_CUT(List<CPart> parts, int startPart, int LineLength, int LineWidth, bool THE_SAME_MAX, bool MaxWay, bool OptiOn, out CLine PreCut)
	{
		double totalMilliseconds = DateTime.Now.TimeOfDay.TotalMilliseconds;
		CLine cLine = new CLine();
		CSnip cSnip = new CSnip();
		cLine.Snips = new List<CSnip>();
		cLine.PartIDs = new List<int>();
		cLine.Parts_Crds = new List<Crd>();
		Get_ID_LD_WD(parts, startPart, out var _, out var _, out var WD);
		bool rez = false;
		cLine.W = WD;
		cLine.L = LineLength;
		cSnip.CRD = new Crd();
		cSnip.CRD.X = 0;
		cSnip.CRD.Y = 0;
		cSnip.L = cLine.L;
		cSnip.W = cLine.W;
		cLine.Snips.Add(cSnip);
		int io = 0;
		if (THE_SAME_MAX)
		{
			int Min_L;
			int Total_Length;
			List<int> fixWidth = Get_Parts_with_FixWidth(parts, WD, cLine.W, out Min_L, out Total_Length);
			List<int> startParts_for_Line_LENGTH_CUT = GetStartParts_for_Line_LENGTH_CUT(parts, fixWidth, cLine.L, Min_L);
			io = FindSmallSnip(cLine.Snips, parts);
			for (int i = 0; i < startParts_for_Line_LENGTH_CUT.Count; i++)
			{
				Place_Part_to_Line(cLine, parts, startParts_for_Line_LENGTH_CUT[i], io, rez);
			}
		}
		else
		{
			Place_Part_to_Line(cLine, parts, startPart, io, rez);
			bool flag = false;
			int num = -1;
			io = FindSmallSnip(cLine.Snips, parts);
			if (io != -1)
			{
				while (!flag)
				{
					int l = cLine.Snips[io].L;
					int w = cLine.Snips[io].W;
					num = Find_THE_SAME_WIDTH_part(parts, l, w);
					if (num != -1)
					{
						Place_Part_to_Line(cLine, parts, num, io, rez);
					}
					else
					{
						flag = true;
					}
				}
			}
		}
		PreCut = CopyLine_WITHOUT_MARKS(cLine);
		Continue_Line_LENGTH_CUT(cLine, parts, MaxWay, OptiOn);
		SET_OFF_Parts_in_Line(parts, cLine);
		CT.T_MakeLine_LENGTH_CUT += (DateTime.Now.TimeOfDay.TotalMilliseconds - totalMilliseconds) / 1000.0;
		CT.C_MakeLine_LENGTH_CUT++;
		return cLine;
	}

	private CLine CopyLine_WITHOUT_MARKS(CLine LINE)
	{
		double totalMilliseconds = DateTime.Now.TimeOfDay.TotalMilliseconds;
		CLine cLine = new CLine
		{
			Snips = new List<CSnip>(),
			PartIDs = new List<int>(),
			Parts_Crds = new List<Crd>(),
			L = LINE.L,
			W = LINE.W,
			Parts_Sq = LINE.Parts_Sq
		};
		for (int i = 0; i < LINE.PartIDs.Count; i++)
		{
			cLine.PartIDs.Add(LINE.PartIDs[i]);
			cLine.Parts_Crds.Add(LINE.Parts_Crds[i]);
		}
		for (int j = 0; j < LINE.Snips.Count; j++)
		{
			CSnip cSnip = new CSnip();
			cSnip.L = LINE.Snips[j].L;
			cSnip.W = LINE.Snips[j].W;
			cSnip.CRD = new Crd();
			cSnip.CRD.X = LINE.Snips[j].CRD.X;
			cSnip.CRD.Y = LINE.Snips[j].CRD.Y;
			cLine.Snips.Add(cSnip);
		}
		CT.T_CopyLine_WITHOUT_MARKS += (DateTime.Now.TimeOfDay.TotalMilliseconds - totalMilliseconds) / 1000.0;
		CT.C_CopyLine_WITHOUT_MARKS++;
		return cLine;
	}

	private bool FastFindFirstPart(List<CPart> parts, int LO, int WO)
	{
		double totalMilliseconds = DateTime.Now.TimeOfDay.TotalMilliseconds;
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
		CT.T_FastFindFirstPart += (DateTime.Now.TimeOfDay.TotalMilliseconds - totalMilliseconds) / 1000.0;
		CT.C_FastFindFirstPart++;
		return result;
	}

	private int Find_WIDTH_part(List<CPart> parts, int LO, int WO, bool Max_W)
	{
		double totalMilliseconds = DateTime.Now.TimeOfDay.TotalMilliseconds;
		int result = -1;
		int num = 0;
		double num2 = 0.0;
		for (int i = 0; i < parts.Count; i++)
		{
			if (parts[i].Plased >= parts[i].Qty)
			{
				continue;
			}
			if (!parts[i].Turn)
			{
				if (LO >= parts[i].L && WO >= parts[i].W)
				{
					if (parts[i].W > num)
					{
						num2 = parts[i].Sq;
						num = parts[i].W;
						result = i;
					}
					else if (parts[i].W == num && (long)(parts[i].Sq - num2) > 0)
					{
						num2 = parts[i].Sq;
						num = parts[i].W;
						result = i;
					}
				}
			}
			else if (parts[i].Turn)
			{
				int num3 = 0;
				if (LO >= parts[i].L && WO >= parts[i].W && LO >= parts[i].W && WO >= parts[i].L)
				{
					num3 = (Max_W ? ((parts[i].L < parts[i].W) ? parts[i].W : parts[i].L) : ((parts[i].L < parts[i].W) ? parts[i].L : parts[i].W));
				}
				else if (LO >= parts[i].L && WO >= parts[i].W && (LO < parts[i].W || WO < parts[i].L))
				{
					num3 = parts[i].W;
				}
				else if ((LO < parts[i].L || WO < parts[i].W) && LO >= parts[i].W && WO >= parts[i].L)
				{
					num3 = parts[i].L;
				}
				if (num3 > num)
				{
					num2 = parts[i].Sq;
					num = num3;
					result = ((parts[i].W != num3) ? (-1 * i - 2) : i);
				}
				else if (num3 == num && (long)(parts[i].Sq - num2) > 0)
				{
					num2 = parts[i].Sq;
					num = num3;
					result = ((parts[i].W != num3) ? (-1 * i - 2) : i);
				}
			}
		}
		CT.T_Find_WIDTH_part += (DateTime.Now.TimeOfDay.TotalMilliseconds - totalMilliseconds) / 1000.0;
		CT.C_Find_WIDTH_part++;
		return result;
	}

	private int Find_THE_SAME_WIDTH_part(List<CPart> parts, int LO, int WO)
	{
		double totalMilliseconds = DateTime.Now.TimeOfDay.TotalMilliseconds;
		int result = -1;
		double num = 0.0;
		for (int i = 0; i < parts.Count; i++)
		{
			if (parts[i].Plased >= parts[i].Qty)
			{
				continue;
			}
			if (!parts[i].Turn)
			{
				if (LO >= parts[i].L && WO == parts[i].W && (long)(parts[i].Sq - num) > 0)
				{
					num = parts[i].Sq;
					result = i;
				}
			}
			else if (parts[i].Turn)
			{
				int num2 = 0;
				if (LO >= parts[i].L && WO == parts[i].W)
				{
					num2 = parts[i].W;
				}
				else if (LO >= parts[i].W && WO == parts[i].L)
				{
					num2 = parts[i].L;
				}
				if (num2 > 0 && (long)(parts[i].Sq - num) > 0)
				{
					num = parts[i].Sq;
					result = ((parts[i].W != num2) ? (-1 * i - 2) : i);
				}
			}
		}
		CT.T_Find_THE_SAME_WIDTH_part += (DateTime.Now.TimeOfDay.TotalMilliseconds - totalMilliseconds) / 1000.0;
		CT.C_Find_THE_SAME_WIDTH_part++;
		return result;
	}

	private void Place_Part_to_Line(CLine line, List<CPart> parts, int part_id, int io, bool _rez)
	{
		double totalMilliseconds = DateTime.Now.TimeOfDay.TotalMilliseconds;
		Get_ID_LD_WD(parts, part_id, out var ID, out var LD, out var WD);
		line.Parts_Sq += parts[ID].Sq;
		line.PartIDs.Add(part_id);
		Crd crd = new Crd();
		crd.X = line.Snips[io].CRD.X;
		crd.Y = line.Snips[io].CRD.Y;
		crd.id_in_order = parts[ID].iD_in_Order;
		line.Parts_Crds.Add(crd);
		parts[ID].Plased++;
		int l = line.Snips[io].L;
		int w = line.Snips[io].W;
		if (l > LD && w > WD)
		{
			if (_rez)
			{
				int num = l - LD - B;
				int num2 = w;
				int x = line.Snips[io].CRD.X + LD + B;
				int y = line.Snips[io].CRD.Y;
				if (FastFindFirstPart(parts, num, num2))
				{
					CSnip item = Create_CSnip(x, y, num, num2);
					line.Snips.Add(item);
					Resize_CSnip(line.Snips[io], line.Snips[io].CRD.X, line.Snips[io].CRD.Y + WD + B, LD, w - WD - B);
				}
				else
				{
					CSnip item2 = Create_CSnip(x, y, num, WD);
					line.Snips.Add(item2);
					Resize_CSnip(line.Snips[io], line.Snips[io].CRD.X, line.Snips[io].CRD.Y + WD + B, l, w - WD - B);
				}
			}
			else
			{
				int num3 = l;
				int num4 = w - WD - B;
				int x2 = line.Snips[io].CRD.X;
				int y2 = line.Snips[io].CRD.Y + WD + B;
				if (FastFindFirstPart(parts, num3, num4))
				{
					CSnip item3 = Create_CSnip(x2, y2, num3, num4);
					line.Snips.Add(item3);
					Resize_CSnip(line.Snips[io], line.Snips[io].CRD.X + LD + B, line.Snips[io].CRD.Y, l - LD - B, WD);
				}
				else
				{
					CSnip item4 = Create_CSnip(x2, y2, LD, num4);
					line.Snips.Add(item4);
					Resize_CSnip(line.Snips[io], line.Snips[io].CRD.X + LD + B, line.Snips[io].CRD.Y, l - LD - B, w);
				}
			}
		}
		else if (LD == l && WD < w)
		{
			Resize_CSnip(line.Snips[io], line.Snips[io].CRD.X, line.Snips[io].CRD.Y + WD + B, l, w - WD - B);
		}
		else if (LD < l && WD == w)
		{
			Resize_CSnip(line.Snips[io], line.Snips[io].CRD.X + LD + B, line.Snips[io].CRD.Y, l - LD - B, w);
		}
		else if (LD == l && WD == w)
		{
			Resize_CSnip(line.Snips[io], line.Snips[io].CRD.X, line.Snips[io].CRD.Y, 0, 0);
		}
		CT.T_Place_Part_to_Line += (DateTime.Now.TimeOfDay.TotalMilliseconds - totalMilliseconds) / 1000.0;
		CT.C_Place_Part_to_Line++;
	}

	private void Place_2_Parts_to_Line(CLine line, List<CPart> parts, int[] _2parts, int io)
	{
		double totalMilliseconds = DateTime.Now.TimeOfDay.TotalMilliseconds;
		int l = line.Snips[io].L;
		int w = line.Snips[io].W;
		int x = line.Snips[io].CRD.X;
		int y = line.Snips[io].CRD.Y;
		_ = (l - B) / 2;
		_ = (w - B) / 2;
		int num = 0;
		int num2 = 0;
		int num3 = _2parts[0];
		int num4 = _2parts[1];
		int num5;
		int num6;
		if (num3 >= 0)
		{
			num5 = parts[num3].L;
			num6 = parts[num3].W;
		}
		else
		{
			num3 = num3 * -1 - 2;
			num5 = parts[num3].W;
			num6 = parts[num3].L;
		}
		line.Parts_Sq += parts[num3].Sq;
		if (num4 != -1)
		{
			if (num4 >= 0)
			{
				num = parts[num4].L;
				num2 = parts[num4].W;
			}
			else
			{
				num4 = num4 * -1 - 2;
				num = parts[num4].W;
				num2 = parts[num4].L;
			}
			line.Parts_Sq += parts[num4].Sq;
		}
		Crd crd = new Crd();
		crd.X = line.Snips[io].CRD.X;
		crd.Y = line.Snips[io].CRD.Y;
		crd.id_in_order = parts[num3].iD_in_Order;
		line.Parts_Crds.Add(crd);
		line.PartIDs.Add(_2parts[0]);
		parts[num3].Plased++;
		if (num4 != -1)
		{
			int num7 = l - B - num5;
			int num8 = w;
			int num9 = l;
			int num10 = w - B - num6;
			bool flag = false;
			bool flag2 = false;
			if (num <= num7 && num2 <= num8)
			{
				flag = true;
			}
			if (num <= num9 && num2 <= num10)
			{
				flag2 = true;
			}
			int num11 = -1;
			int num12 = -1;
			double num13 = line.Snips[io].Sq - parts[num3].Sq - parts[num4].Sq;
			double num14 = line.Snips[io].Sq - parts[num3].Sq - parts[num4].Sq;
			if (flag)
			{
				if (num6 > num2)
				{
					int lO = l;
					int wO = w - num6 - B;
					int lO2 = num;
					int wO2 = num6 - num2 - B;
					int lO3 = l - num5 - num - 2 * B;
					int wO3 = num6;
					double num15 = GetSqPartsForSnips(parts, lO, wO, lO2, wO2, lO3, wO3);
					lO = num5;
					wO = w - num6 - B;
					lO2 = num;
					wO2 = w - num2 - B;
					lO3 = l - num5 - num - 2 * B;
					wO3 = w;
					double sqPartsForSnips = GetSqPartsForSnips(parts, lO, wO, lO2, wO2, lO3, wO3);
					lO = num5;
					wO = w - num6 - B;
					lO2 = l - num5 - B;
					wO2 = w - num2 - B;
					lO3 = l - num5 - num - 2 * B;
					wO3 = num2;
					double num16 = GetSqPartsForSnips(parts, lO, wO, lO2, wO2, lO3, wO3);
					lO = l;
					wO = w - num6 - B;
					lO2 = l - num5 - B;
					wO2 = num6 - num2 - B;
					lO3 = l - num5 - num - 2 * B;
					wO3 = num2;
					double sqPartsForSnips2 = GetSqPartsForSnips(parts, lO, wO, lO2, wO2, lO3, wO3);
					int num17 = -1;
					if ((long)(num15 * 100.0) == 0L && (long)(sqPartsForSnips * 100.0) == 0L)
					{
						num17 = -1;
					}
					else if ((long)(num15 * 100.0) != 0L && (long)(sqPartsForSnips * 100.0) == 0L)
					{
						num17 = 1;
					}
					else if ((long)(num15 * 100.0) == 0L && (long)(sqPartsForSnips * 100.0) != 0L)
					{
						num17 = 2;
						num15 = sqPartsForSnips;
					}
					else if ((long)(num15 * 100.0) != 0L && (long)(sqPartsForSnips * 100.0) != 0L)
					{
						if ((long)((num15 - sqPartsForSnips) * 100.0) >= 0)
						{
							num17 = 1;
						}
						else
						{
							num17 = 2;
							num15 = sqPartsForSnips;
						}
					}
					int num18 = -1;
					if ((long)(num16 * 100.0) == 0L && (long)(sqPartsForSnips2 * 100.0) == 0L)
					{
						num18 = -1;
					}
					else if ((long)(num16 * 100.0) != 0L && (long)(sqPartsForSnips2 * 100.0) == 0L)
					{
						num18 = 3;
					}
					else if ((long)(num16 * 100.0) == 0L && (long)(sqPartsForSnips2 * 100.0) != 0L)
					{
						num18 = 4;
						num16 = sqPartsForSnips2;
					}
					else if ((long)(num16 * 100.0) != 0L && (long)(sqPartsForSnips2 * 100.0) != 0L)
					{
						if ((long)((num16 - sqPartsForSnips2) * 100.0) >= 0)
						{
							num18 = 3;
						}
						else
						{
							num18 = 4;
							num16 = sqPartsForSnips2;
						}
					}
					if (num17 == -1 && num18 == -1)
					{
						num17 = 1;
						num15 = num15;
					}
					else if (num17 != -1 && num18 == -1)
					{
						num17 = num17;
						num15 = num15;
					}
					else if (num17 == -1 && num18 != -1)
					{
						num17 = num18;
						num15 = num16;
					}
					else if (num17 != -1 && num18 != -1 && (long)((num15 - num16) * 100.0) < 0)
					{
						num17 = num18;
						num15 = num16;
					}
					num13 -= num15;
					num11 = num17;
				}
				else
				{
					int lO4 = l;
					int wO4 = w - num6 - B;
					int lO5 = l - num5 - num - 2 * B;
					int wO5 = num6;
					int lO6 = 0;
					int wO6 = 0;
					double num19 = GetSqPartsForSnips(parts, lO4, wO4, lO5, wO5, lO6, wO6);
					lO4 = num5;
					wO4 = w - num6 - B;
					lO5 = num;
					wO5 = w - num2 - B;
					lO6 = l - num5 - num - 2 * B;
					wO6 = w;
					double sqPartsForSnips3 = GetSqPartsForSnips(parts, lO4, wO4, lO5, wO5, lO6, wO6);
					int num20 = -1;
					if ((long)(num19 * 100.0) == 0L && (long)(sqPartsForSnips3 * 100.0) == 0L)
					{
						num20 = -1;
					}
					else if ((long)(num19 * 100.0) != 0L && (long)(sqPartsForSnips3 * 100.0) == 0L)
					{
						num20 = 5;
					}
					else if ((long)(num19 * 100.0) == 0L && (long)(sqPartsForSnips3 * 100.0) != 0L)
					{
						num20 = 6;
						num19 = sqPartsForSnips3;
					}
					else if ((long)(num19 * 100.0) != 0L && (long)(sqPartsForSnips3 * 100.0) != 0L)
					{
						if ((long)((num19 - sqPartsForSnips3) * 100.0) >= 0)
						{
							num20 = 5;
						}
						else
						{
							num20 = 6;
							num19 = sqPartsForSnips3;
						}
					}
					if (num20 == -1)
					{
						num19 = 0.0;
						num20 = 5;
					}
					num13 -= num19;
					num11 = num20;
				}
			}
			if (flag2)
			{
				if (num5 > num)
				{
					int lO7 = l - num5 - B;
					int wO7 = w;
					int lO8 = num5 - num - B;
					int wO8 = num2;
					int lO9 = num5;
					int wO9 = w - num6 - num2 - 2 * B;
					double num21 = GetSqPartsForSnips(parts, lO7, wO7, lO8, wO8, lO9, wO9);
					lO7 = l - num5 - B;
					wO7 = w;
					lO8 = num5 - num - B;
					wO8 = w - num6 - B;
					lO9 = num;
					wO9 = w - num6 - num2 - 2 * B;
					double sqPartsForSnips4 = GetSqPartsForSnips(parts, lO7, wO7, lO8, wO8, lO9, wO9);
					lO7 = l - num5 - B;
					wO7 = num6;
					lO8 = l - num - B;
					wO8 = num2;
					lO9 = l;
					wO9 = w - num6 - num2 - 2 * B;
					double num22 = GetSqPartsForSnips(parts, lO7, wO7, lO8, wO8, lO9, wO9);
					lO7 = l - num5 - B;
					wO7 = num6;
					lO8 = l - num - B;
					wO8 = w - num6 - B;
					lO9 = num;
					wO9 = w - num6 - num2 - 2 * B;
					double sqPartsForSnips5 = GetSqPartsForSnips(parts, lO7, wO7, lO8, wO8, lO9, wO9);
					int num23 = -1;
					if ((long)(num21 * 100.0) == 0L && (long)(sqPartsForSnips4 * 100.0) == 0L)
					{
						num23 = -1;
					}
					else if ((long)(num21 * 100.0) != 0L && (long)(sqPartsForSnips4 * 100.0) == 0L)
					{
						num23 = 1;
					}
					else if ((long)(num21 * 100.0) == 0L && (long)(sqPartsForSnips4 * 100.0) != 0L)
					{
						num23 = 2;
						num21 = sqPartsForSnips4;
					}
					else if ((long)(num21 * 100.0) != 0L && (long)(sqPartsForSnips4 * 100.0) != 0L)
					{
						if ((long)((num21 - sqPartsForSnips4) * 100.0) >= 0)
						{
							num23 = 1;
						}
						else
						{
							num23 = 2;
							num21 = sqPartsForSnips4;
						}
					}
					int num24 = -1;
					if ((long)(num22 * 100.0) == 0L && (long)(sqPartsForSnips5 * 100.0) == 0L)
					{
						num24 = -1;
					}
					else if ((long)(num22 * 100.0) != 0L && (long)(sqPartsForSnips5 * 100.0) == 0L)
					{
						num24 = 3;
					}
					else if ((long)(num22 * 100.0) == 0L && (long)(sqPartsForSnips5 * 100.0) != 0L)
					{
						num24 = 4;
						num22 = sqPartsForSnips5;
					}
					else if ((long)(num22 * 100.0) != 0L && (long)(sqPartsForSnips5 * 100.0) != 0L)
					{
						if ((long)((num22 - sqPartsForSnips5) * 100.0) >= 0)
						{
							num24 = 3;
						}
						else
						{
							num24 = 4;
							num22 = sqPartsForSnips5;
						}
					}
					if (num23 == -1 && num24 == -1)
					{
						num23 = 1;
						num21 = num21;
					}
					else if (num23 != -1 && num24 == -1)
					{
						num23 = num23;
						num21 = num21;
					}
					else if (num23 == -1 && num24 != -1)
					{
						num23 = num24;
						num21 = num22;
					}
					else if (num23 != -1 && num24 != -1 && (long)((num21 - num22) * 100.0) < 0)
					{
						num23 = num24;
						num21 = num22;
					}
					num14 -= num21;
					num12 = num23;
				}
				else
				{
					int lO10 = l - num5 - B;
					int wO10 = w;
					int lO11 = num;
					int wO11 = w - num6 - num2 - 2 * B;
					int lO12 = 0;
					int wO12 = 0;
					double num25 = GetSqPartsForSnips(parts, lO10, wO10, lO11, wO11, lO12, wO12);
					lO10 = l - num5 - B;
					wO10 = num6;
					lO11 = l - num5 - B;
					wO11 = num2;
					lO12 = l;
					wO12 = w - num6 - num2 - 2 * B;
					double sqPartsForSnips6 = GetSqPartsForSnips(parts, lO10, wO10, lO11, wO11, lO12, wO12);
					int num26 = -1;
					if ((long)(num25 * 100.0) == 0L && (long)(sqPartsForSnips6 * 100.0) == 0L)
					{
						num26 = -1;
					}
					else if ((long)(num25 * 100.0) != 0L && (long)(sqPartsForSnips6 * 100.0) == 0L)
					{
						num26 = 5;
					}
					else if ((long)(num25 * 100.0) == 0L && (long)(sqPartsForSnips6 * 100.0) != 0L)
					{
						num26 = 6;
						num25 = sqPartsForSnips6;
					}
					else if ((long)(num25 * 100.0) != 0L && (long)(sqPartsForSnips6 * 100.0) != 0L)
					{
						if ((long)((num25 - sqPartsForSnips6) * 100.0) >= 0)
						{
							num26 = 5;
						}
						else
						{
							num26 = 6;
							num25 = sqPartsForSnips6;
						}
					}
					if (num26 == -1)
					{
						num25 = 0.0;
						num26 = 5;
					}
					num14 -= num25;
					num12 = num26;
				}
			}
			int num27 = 1;
			if (flag && flag2)
			{
				num27 = (((long)(num14 - num13) == 0L) ? ((l < w) ? (num11 * -1) : num12) : (((long)(num14 - num13) >= 0) ? (num11 * -1) : num12));
			}
			else
			{
				if (flag)
				{
					num27 = num11 * -1;
				}
				if (flag2)
				{
					num27 = num12;
				}
			}
			Crd crd2 = new Crd();
			if (num27 < 0)
			{
				crd2.X = x + num5 + B;
				crd2.Y = y;
			}
			else
			{
				crd2.X = x;
				crd2.Y = y + num6 + B;
			}
			crd2.id_in_order = parts[num4].iD_in_Order;
			line.Parts_Crds.Add(crd2);
			line.PartIDs.Add(_2parts[1]);
			parts[num4].Plased++;
			switch (num27)
			{
			case -1:
				Resize_CSnip(line.Snips[io], x, y + num6 + B, l, w - num6 - B);
				line.Snips.Add(Create_CSnip(x + num5 + B, y + num2 + B, num, num6 - num2 - B));
				line.Snips.Add(Create_CSnip(x + num5 + num + 2 * B, y, l - num5 - num - 2 * B, num6));
				break;
			case -2:
				Resize_CSnip(line.Snips[io], x, y + num6 + B, num5, w - num6 - B);
				line.Snips.Add(Create_CSnip(x + num5 + B, y + num2 + B, num, w - num2 - B));
				line.Snips.Add(Create_CSnip(x + num5 + num + 2 * B, y, l - num5 - num - 2 * B, w));
				break;
			case -3:
				Resize_CSnip(line.Snips[io], x, y + num6 + B, num5, w - num6 - B);
				line.Snips.Add(Create_CSnip(x + num5 + B, y + num2 + B, l - num5 - B, w - num2 - B));
				line.Snips.Add(Create_CSnip(x + num5 + num + 2 * B, y, l - num5 - num - 2 * B, num2));
				break;
			case -4:
				Resize_CSnip(line.Snips[io], x, y + num6 + B, l, w - num6 - B);
				line.Snips.Add(Create_CSnip(x + num5 + B, y + num2 + B, l - num5 - B, num6 - num2 - B));
				line.Snips.Add(Create_CSnip(x + num5 + num + 2 * B, y, l - num5 - num - 2 * B, num2));
				break;
			case -5:
				Resize_CSnip(line.Snips[io], x, y + num6 + B, l, w - num6 - B);
				line.Snips.Add(Create_CSnip(x + num5 + num + 2 * B, y, l - num5 - num - 2 * B, num6));
				break;
			case -6:
				Resize_CSnip(line.Snips[io], x, y + num6 + B, num5, w - num6 - B);
				line.Snips.Add(Create_CSnip(x + num5 + B, y + num2 + B, num, w - num2 - B));
				line.Snips.Add(Create_CSnip(x + num5 + num + 2 * B, y, l - num5 - num - 2 * B, w));
				break;
			case 1:
				Resize_CSnip(line.Snips[io], x + num5 + B, y, l - num5 - B, w);
				line.Snips.Add(Create_CSnip(x + num + B, y + num6 + B, num5 - num - B, num2));
				line.Snips.Add(Create_CSnip(x, y + num6 + num2 + 2 * B, num5, w - num6 - num2 - 2 * B));
				break;
			case 2:
				Resize_CSnip(line.Snips[io], x + num5 + B, y, l - num5 - B, w);
				line.Snips.Add(Create_CSnip(x + num + B, y + num6 + B, num5 - num - B, w - num6 - B));
				line.Snips.Add(Create_CSnip(x, y + num6 + num2 + 2 * B, num, w - num6 - num2 - 2 * B));
				break;
			case 3:
				Resize_CSnip(line.Snips[io], x + num5 + B, y, l - num5 - B, -num6);
				line.Snips.Add(Create_CSnip(x + num + B, y + num6 + B, l - num - B, num2));
				line.Snips.Add(Create_CSnip(x, y + num6 + num2 + 2 * B, l, w - num6 - num2 - 2 * B));
				break;
			case 4:
				Resize_CSnip(line.Snips[io], x + num5 + B, y, l - num5 - B, num6);
				line.Snips.Add(Create_CSnip(x + num + B, y + num6 + B, l - num - B, w - num6 - B));
				line.Snips.Add(Create_CSnip(x, y + num6 + num2 + 2 * B, num, w - num6 - num2 - 2 * B));
				break;
			case 5:
				Resize_CSnip(line.Snips[io], x + num5 + B, y, l - num5 - B, w);
				line.Snips.Add(Create_CSnip(x, y + num6 + num2 + 2 * B, num, w - num6 - num2 - 2 * B));
				break;
			case 6:
				Resize_CSnip(line.Snips[io], x + num5 + B, y, l - num5 - B, num6);
				line.Snips.Add(Create_CSnip(x + num + B, y + num6 + B, l - num - B, num2));
				line.Snips.Add(Create_CSnip(x, y + num6 + num2 + 2 * B, l, w - num6 - num2 - 2 * B));
				break;
			}
		}
		else
		{
			int lO13 = num5;
			int wO13 = w - num6 - B;
			int lO14 = l - num5 - B;
			int wO14 = w;
			double sqPartsForSnips7 = GetSqPartsForSnips(parts, lO13, wO13, lO14, wO14, 0, 0);
			lO13 = l;
			wO13 = w - num6 - B;
			lO14 = l - num5 - B;
			wO14 = num6;
			double sqPartsForSnips8 = GetSqPartsForSnips(parts, lO13, wO13, lO14, wO14, 0, 0);
			int num28 = -1;
			if ((long)(sqPartsForSnips7 * 100.0) == 0L && (long)(sqPartsForSnips8 * 100.0) == 0L)
			{
				num28 = -1;
			}
			else if ((long)(sqPartsForSnips7 * 100.0) != 0L && (long)(sqPartsForSnips8 * 100.0) == 0L)
			{
				num28 = 1;
			}
			else if ((long)(sqPartsForSnips7 * 100.0) == 0L && (long)(sqPartsForSnips8 * 100.0) != 0L)
			{
				num28 = 2;
			}
			else if ((long)(sqPartsForSnips7 * 100.0) != 0L && (long)(sqPartsForSnips8 * 100.0) != 0L)
			{
				num28 = (((long)(sqPartsForSnips7 - sqPartsForSnips8) >= 0) ? 1 : 2);
			}
			if (num28 == -1)
			{
				num28 = 1;
			}
			switch (num28)
			{
			case 1:
				Resize_CSnip(line.Snips[io], x, y + num6 + B, num5, w - num6 - B);
				line.Snips.Add(Create_CSnip(x + num5 + B, y, l - num5 - B, w));
				break;
			case 2:
				Resize_CSnip(line.Snips[io], x, y + num6 + B, l, w - num6 - B);
				line.Snips.Add(Create_CSnip(x + num5 + B, y, l - num5 - B, num6));
				break;
			default:
				Resize_CSnip(line.Snips[io], x, y + num6 + B, num5, w - num6 - B);
				line.Snips.Add(Create_CSnip(x + num5 + B, y, l - num5 - B, w));
				break;
			}
		}
		CT.T_Place_2_Parts_to_Line += (DateTime.Now.TimeOfDay.TotalMilliseconds - totalMilliseconds) / 1000.0;
		CT.C_Place_2_Parts_to_Line++;
	}

	private int[] Check_part_for_last_in_Line(List<CPart> parts, int LO, int WO, bool rez, int id)
	{
		double totalMilliseconds = DateTime.Now.TimeOfDay.TotalMilliseconds;
		int[] array = new int[2] { -1, -1 };
		int num = id;
		int num2;
		int num3;
		if (num >= 0)
		{
			num2 = parts[num].L;
			num3 = parts[num].W;
		}
		else
		{
			num = id * -1 - 2;
			num2 = parts[num].W;
			num3 = parts[num].L;
		}
		double sq = parts[num].Sq;
		int num4 = num2;
		int num5 = WO - num3 - B;
		int num6 = LO - num2 - B;
		int num7 = WO;
		bool flag = false;
		bool flag2 = false;
		if (num4 >= minL && num5 >= minW)
		{
			flag = FastFindFirstPart(parts, num4, num5);
		}
		if (num6 >= minL && num7 >= minW)
		{
			flag2 = FastFindFirstPart(parts, num6, num7);
		}
		if (flag || flag2)
		{
			array[0] = id;
		}
		else
		{
			num4 = LO - num2 - B;
			num5 = num3;
			num6 = LO;
			num7 = WO - num3 - B;
			flag = false;
			flag2 = false;
			if (num4 >= minL && num5 >= minW)
			{
				flag = FastFindFirstPart(parts, num4, num5);
			}
			if (num6 >= minL && num7 >= minW)
			{
				flag2 = FastFindFirstPart(parts, num6, num7);
			}
			if (flag || flag2)
			{
				array[0] = id;
			}
			else
			{
				int[] array2 = Find_2_Parts(parts, LO, WO);
				double num8 = 0.0;
				if (array2[0] != -1)
				{
					num8 = ((array2[0] >= -1) ? (num8 + parts[array2[0]].Sq) : (num8 + parts[array2[0] * -1 - 2].Sq));
				}
				if (array2[1] != -1)
				{
					num8 = ((array2[1] >= -1) ? (num8 + parts[array2[1]].Sq) : (num8 + parts[array2[1] * -1 - 2].Sq));
				}
				if ((long)(num8 - sq) > 0)
				{
					array = array2;
				}
			}
		}
		CT.T_Check_part_for_last_in_Line += (DateTime.Now.TimeOfDay.TotalMilliseconds - totalMilliseconds) / 1000.0;
		CT.C_Check_part_for_last_in_Line++;
		return array;
	}

	private CSnip Create_CSnip(int X, int Y, int length, int width)
	{
		CSnip cSnip = new CSnip();
		cSnip.L = length;
		cSnip.W = width;
		cSnip.CRD = new Crd();
		cSnip.CRD.X = X;
		cSnip.CRD.Y = Y;
		return cSnip;
	}

	private void Resize_CSnip(CSnip snip, int X, int Y, int length, int width)
	{
		snip.L = length;
		snip.W = width;
		snip.CRD.X = X;
		snip.CRD.Y = Y;
	}

	private double GetSqPartsForSnips(List<CPart> parts, int LO1, int WO1, int LO2, int WO2, int LO3, int WO3)
	{
		double totalMilliseconds = DateTime.Now.TimeOfDay.TotalMilliseconds;
		double num = 0.0;
		int num2 = -1;
		int num3 = -1;
		int num4 = -1;
		if (LO1 >= minL && WO1 >= minW)
		{
			num2 = FindMaxSqPart(parts, LO1, WO1);
			if (num2 != -1)
			{
				if (num2 < -1)
				{
					num2 = num2 * -1 - 2;
				}
				num += parts[num2].Sq;
			}
		}
		if (num2 != -1)
		{
			parts[num2].Plased++;
		}
		if (LO2 >= minL && WO2 >= minW)
		{
			num3 = FindMaxSqPart(parts, LO2, WO2);
			if (num3 != -1)
			{
				if (num3 < -1)
				{
					num3 = num3 * -1 - 2;
				}
				num += parts[num3].Sq;
			}
		}
		if (num3 != -1)
		{
			parts[num3].Plased++;
		}
		if (LO3 >= minL && WO3 >= minW)
		{
			num4 = FindMaxSqPart(parts, LO3, WO3);
			if (num4 != -1)
			{
				if (num4 < -1)
				{
					num4 = num4 * -1 - 2;
				}
				num += parts[num4].Sq;
			}
		}
		if (num2 != -1)
		{
			parts[num2].Plased--;
		}
		if (num3 != -1)
		{
			parts[num3].Plased--;
		}
		CT.T_GetSqPartsForSnips += (DateTime.Now.TimeOfDay.TotalMilliseconds - totalMilliseconds) / 1000.0;
		CT.C_GetSqPartsForSnips++;
		return num;
	}

	private int FindMaxSqPart(List<CPart> parts, int LO, int WO)
	{
		double totalMilliseconds = DateTime.Now.TimeOfDay.TotalMilliseconds;
		int result = -1;
		double num = 0.0;
		if (LO > 0 && WO > 0)
		{
			for (int i = 0; i < parts.Count; i++)
			{
				if (parts[i].Plased >= parts[i].Qty)
				{
					continue;
				}
				if (parts[i].L <= LO && parts[i].W <= WO)
				{
					if ((long)(parts[i].Sq - num) > 0)
					{
						result = i;
						num = parts[i].Sq;
					}
				}
				else if (parts[i].Turn && parts[i].L <= WO && parts[i].W <= LO && (long)(parts[i].Sq - num) > 0)
				{
					result = i * -1 - 2;
					num = parts[i].Sq;
				}
			}
		}
		CT.T_FindMaxSqPart += (DateTime.Now.TimeOfDay.TotalMilliseconds - totalMilliseconds) / 1000.0;
		CT.C_FindMaxSqPart++;
		return result;
	}

	private int FindMaxSqPart(List<CPart> parts, int LO, int WO, int krome)
	{
		double totalMilliseconds = DateTime.Now.TimeOfDay.TotalMilliseconds;
		int result = -1;
		double num = 0.0;
		if (krome < -1)
		{
			krome = krome * -1 - 2;
		}
		if (LO > 0 && WO > 0)
		{
			for (int i = 0; i < parts.Count; i++)
			{
				int num2 = parts[i].Qty;
				if (i == krome)
				{
					num2--;
				}
				if (parts[i].Plased >= num2)
				{
					continue;
				}
				if (parts[i].L <= LO && parts[i].W <= WO)
				{
					if ((long)(parts[i].Sq - num) > 0)
					{
						result = i;
						num = parts[i].Sq;
					}
				}
				else if (parts[i].Turn && parts[i].L <= WO && parts[i].W <= LO && (long)(parts[i].Sq - num) > 0)
				{
					result = i * -1 - 2;
					num = parts[i].Sq;
				}
			}
		}
		CT.T_FindMaxSqPart_krome += (DateTime.Now.TimeOfDay.TotalMilliseconds - totalMilliseconds) / 1000.0;
		CT.C_FindMaxSqPart_krome++;
		return result;
	}

	private int FindSmallSnip(List<CSnip> snips, List<CPart> parts)
	{
		_ = DateTime.Now.TimeOfDay.TotalMilliseconds;
		int result = -1;
		double num = 100000000000.0;
		for (int i = 0; i < snips.Count; i++)
		{
			if ((long)(num - snips[i].Sq) > 0 && FastFindFirstPart(parts, snips[i].L, snips[i].W))
			{
				num = snips[i].Sq;
				result = i;
			}
		}
		return result;
	}

	private int[] Find_2_Parts(List<CPart> parts, int LO, int WO)
	{
		double totalMilliseconds = DateTime.Now.TimeOfDay.TotalMilliseconds;
		int[] array = new int[3] { -1, -1, 0 };
		int[] array2 = new int[3] { -1, -1, 1 };
		double num = 0.0;
		double num2 = 0.0;
		double num3 = 0.0;
		double num4 = 0.0;
		double num5 = 0.0;
		double num6 = 0.0;
		int num7 = -1;
		for (int i = 0; i < parts.Count; i++)
		{
			num = 0.0;
			num2 = 0.0;
			num3 = 0.0;
			num4 = 0.0;
			if (parts[i].Plased >= parts[i].Qty)
			{
				continue;
			}
			if (LO >= parts[i].L && WO >= parts[i].W)
			{
				num = parts[i].Sq;
				int num8 = LO - B - parts[i].L;
				if (num8 >= minL)
				{
					num7 = FindMaxSqPart(parts, num8, WO, i);
					num2 = ((num7 == -1) ? 0.0 : ((num7 >= -1) ? parts[num7].Sq : parts[num7 * -1 - 2].Sq));
				}
				else
				{
					num7 = -1;
					num2 = 0.0;
				}
				if ((long)(num5 - (num + num2)) < 0)
				{
					num5 = num + num2;
					array[0] = i;
					array[1] = num7;
				}
			}
			if (parts[i].Turn && WO >= parts[i].L && LO >= parts[i].W)
			{
				num = parts[i].Sq;
				int num9 = LO - B - parts[i].W;
				if (num9 >= minL)
				{
					num7 = FindMaxSqPart(parts, num9, WO, i);
					num2 = ((num7 == -1) ? 0.0 : ((num7 >= -1) ? parts[num7].Sq : parts[num7 * -1 - 2].Sq));
				}
				else
				{
					num7 = -1;
					num2 = 0.0;
				}
				if ((long)(num5 - (num + num2)) < 0)
				{
					num5 = num + num2;
					array[0] = i * -1 - 2;
					array[1] = num7;
				}
			}
			if (LO >= parts[i].L && WO >= parts[i].W)
			{
				num3 = parts[i].Sq;
				int num10 = WO - B - parts[i].W;
				if (num10 >= minL)
				{
					num7 = FindMaxSqPart(parts, LO, num10, i);
					num4 = ((num7 == -1) ? 0.0 : ((num7 >= -1) ? parts[num7].Sq : parts[num7 * -1 - 2].Sq));
				}
				else
				{
					num7 = -1;
					num4 = 0.0;
				}
				if ((long)(num6 - (num3 + num4)) < 0)
				{
					num6 = num3 + num4;
					array2[0] = i;
					array2[1] = num7;
				}
			}
			else if (parts[i].Turn && WO >= parts[i].L && LO >= parts[i].W)
			{
				num3 = parts[i].Sq;
				int num11 = WO - B - parts[i].L;
				if (num11 >= minL)
				{
					num7 = FindMaxSqPart(parts, LO, num11, i);
					num4 = ((num7 == -1) ? 0.0 : ((num7 >= -1) ? parts[num7].Sq : parts[num7 * -1 - 2].Sq));
				}
				else
				{
					num7 = -1;
					num4 = 0.0;
				}
				if ((long)(num6 - (num3 + num4)) < 0)
				{
					num6 = num3 + num4;
					array2[0] = i * -1 - 2;
					array2[1] = num7;
				}
			}
		}
		if (array[0] != -1 && array[1] != -1)
		{
			int num12 = array[0];
			int num13;
			if (num12 < -1)
			{
				num12 = num12 * -1 - 2;
				num13 = parts[num12].L;
			}
			else
			{
				num13 = parts[num12].W;
			}
			int num14 = array[1];
			int num15;
			if (num14 < -1)
			{
				num14 = num14 * -1 - 2;
				num15 = parts[num14].L;
			}
			else
			{
				num15 = parts[num14].W;
			}
			if (num15 > num13)
			{
				int num16 = array[0];
				array[0] = array[1];
				array[1] = num16;
			}
		}
		if (array2[0] != -1 && array2[1] != -1)
		{
			int num17 = array2[0];
			int num18;
			if (num17 < -1)
			{
				num17 = num17 * -1 - 2;
				num18 = parts[num17].W;
			}
			else
			{
				num18 = parts[num17].L;
			}
			int num19 = array2[1];
			int num20;
			if (num19 < -1)
			{
				num19 = num19 * -1 - 2;
				num20 = parts[num19].W;
			}
			else
			{
				num20 = parts[num19].L;
			}
			if (num20 > num18)
			{
				int num21 = array2[0];
				array2[0] = array2[1];
				array2[1] = num21;
			}
		}
		CT.T_Find_2_Parts += (DateTime.Now.TimeOfDay.TotalMilliseconds - totalMilliseconds) / 1000.0;
		CT.C_Find_2_Parts++;
		if ((long)(num5 - num6) > 0)
		{
			return array;
		}
		return array2;
	}

	private int[] Find_Zamena_Lines_LENGTH_CUT(List<CLine> Lines, int WO, int Minimal_W, out bool check)
	{
		double totalMilliseconds = DateTime.Now.TimeOfDay.TotalMilliseconds;
		int[] array = new int[3] { -1, -1, -1 };
		check = false;
		int num = 0;
		for (int i = 0; i < Lines.Count; i++)
		{
			int w = Lines[i].W;
			if (Lines[i].onSheet || WO < w)
			{
				continue;
			}
			if (w > num)
			{
				array[0] = i;
				array[1] = -1;
				array[2] = -1;
				num = w;
				check = true;
			}
			else if (w == num)
			{
				double num2 = 0.0;
				for (int j = 0; j < 3; j++)
				{
					if (array[j] != -1)
					{
						num2 += Lines[array[j]].Parts_Sq;
					}
				}
				if ((int)(Lines[i].Parts_Sq - num2) >= 0)
				{
					array[0] = i;
					array[1] = -1;
					array[2] = -1;
					num = w;
					check = true;
				}
			}
			if (WO - w - B - Minimal_W < 0)
			{
				continue;
			}
			for (int k = i + 1; k < Lines.Count; k++)
			{
				int w2 = Lines[k].W;
				if (Lines[k].onSheet || WO < w2)
				{
					continue;
				}
				if (WO - w - B - w2 >= 0)
				{
					if (w + w2 - num > 0)
					{
						array[0] = i;
						array[1] = k;
						array[2] = -1;
						num = w + w2;
						check = true;
					}
					else if (w + w2 - num == 0)
					{
						double num3 = 0.0;
						for (int l = 0; l < 3; l++)
						{
							if (array[l] != -1)
							{
								num3 += Lines[array[l]].Parts_Sq;
							}
						}
						if ((int)(Lines[i].Parts_Sq + Lines[k].Parts_Sq - num3) >= 0)
						{
							array[0] = i;
							array[1] = k;
							array[2] = -1;
							num = w + w2;
							check = true;
						}
					}
				}
				if (WO - w - B - w2 - B - Minimal_W < 0)
				{
					continue;
				}
				for (int m = k + 1; m < Lines.Count; m++)
				{
					int w3 = Lines[m].W;
					if (Lines[m].onSheet || WO < w3 || WO - w - B - w2 - B - w3 < 0)
					{
						continue;
					}
					if (w + w2 + w3 - num > 0)
					{
						array[0] = i;
						array[1] = k;
						array[2] = m;
						num = w + w2 + w3;
						check = true;
					}
					else
					{
						if (w + w2 + w3 - num != 0)
						{
							continue;
						}
						double num4 = 0.0;
						for (int n = 0; n < 3; n++)
						{
							if (array[n] != -1)
							{
								num4 += Lines[array[n]].Parts_Sq;
							}
						}
						if ((int)(Lines[i].Parts_Sq + Lines[k].Parts_Sq + Lines[m].Parts_Sq - num4) >= 0)
						{
							array[0] = i;
							array[1] = k;
							array[2] = m;
							num = w + w2 + w3;
							check = true;
						}
					}
				}
			}
		}
		CT.T_Find_Zamena_Lines_LENGTH_CUT += (DateTime.Now.TimeOfDay.TotalMilliseconds - totalMilliseconds) / 1000.0;
		CT.C_Find_Zamena_Lines_LENGTH_CUT++;
		return array;
	}

	private int[] Find_Zamena_PARTS_LENGTH_CUT(List<int> Fix, List<CPart> parts, int WO, int max_L, int Minimal_L, out bool check)
	{
		double totalMilliseconds = DateTime.Now.TimeOfDay.TotalMilliseconds;
		int[] array = new int[3] { -1, -1, -1 };
		check = false;
		for (int i = 0; i < Fix.Count; i++)
		{
			int num = ((Fix[i] <= -1) ? parts[Fix[i] * -1 - 2].W : parts[Fix[i]].L);
			if (WO < num)
			{
				continue;
			}
			if (num > max_L)
			{
				array[0] = i;
				array[1] = -1;
				array[2] = -1;
				max_L = num;
				check = true;
			}
			if (WO - num - B - Minimal_L < 0)
			{
				continue;
			}
			for (int j = i + 1; j < Fix.Count; j++)
			{
				int num2 = ((Fix[j] <= -1) ? parts[Fix[j] * -1 - 2].W : parts[Fix[j]].L);
				if (WO - num - B - num2 >= 0 && num + num2 - max_L > 0)
				{
					array[0] = i;
					array[1] = j;
					array[2] = -1;
					max_L = num + num2;
					check = true;
				}
				if (WO - num - B - num2 - B - Minimal_L < 0)
				{
					continue;
				}
				for (int k = j + 1; k < Fix.Count; k++)
				{
					int num3 = ((Fix[k] <= -1) ? parts[Fix[k] * -1 - 2].W : parts[Fix[k]].L);
					if (WO - num - B - num2 - B - num3 >= 0 && num + num2 + num3 - max_L > 0)
					{
						array[0] = i;
						array[1] = j;
						array[2] = k;
						max_L = num + num2 + num3;
						check = true;
					}
				}
			}
		}
		CT.T_Find_Zamena_PARTS_LENGTH_CUT += (DateTime.Now.TimeOfDay.TotalMilliseconds - totalMilliseconds) / 1000.0;
		CT.C_Find_Zamena_PARTS_LENGTH_CUT++;
		return array;
	}

	private void Write_Sheets_to_Order_LENGTH_CUT(Order o, List<CSheet> Ss)
	{
		for (int i = 0; i < Ss.Count; i++)
		{
			int p = P;
			int num = P;
			while (Ss[i].Lines.Count > 0)
			{
				int num2 = 0;
				int index = -1;
				for (int j = 0; j < Ss[i].Lines.Count; j++)
				{
					if (Ss[i].Lines[j].W > num2 || (Ss[i].Lines[j].W == num2 && (int)((Ss[i].Lines[j].Parts_Sq - Ss[i].Lines[index].Parts_Sq) * 100.0) > 0))
					{
						index = j;
						num2 = Ss[i].Lines[j].W;
					}
				}
				for (int k = 0; k < Ss[i].Lines[index].PartIDs.Count; k++)
				{
					bool isTurn = false;
					int index2;
					if (Ss[i].Lines[index].PartIDs[k] < -1)
					{
						index2 = Ss[i].Lines[index].PartIDs[k] * -1 - 2;
						isTurn = true;
					}
					else
					{
						index2 = Ss[i].Lines[index].PartIDs[k];
					}
					Crd crd = Ss[i].Lines[index].Parts_Crds[k];
					CPart cPart = CPARTS[index2];
					Part part = o.Parts[cPart.iD_in_Order];
					int nPlased = part.nPlased;
					part.Coords[nPlased].X = (p + crd.X) / 10;
					part.Coords[nPlased].Y = (num + crd.Y) / 10;
					part.Coords[nPlased].isTurn = isTurn;
					part.Coords[nPlased].list = i + 1;
					part.Coords[nPlased].nlist = -1;
					part.Coords[nPlased].Cutted = true;
					part.Coords[nPlased].onList = true;
					part.nPlased++;
					o.PartsPlased++;
				}
				for (int l = 0; l < Ss[i].Lines[index].Snips.Count; l++)
				{
					CSnip cSnip = Ss[i].Lines[index].Snips[l];
					Snip snip = new Snip();
					snip.Length_mm = cSnip.L / 10;
					snip.Width_mm = cSnip.W / 10;
					snip.onList = true;
					snip.Sq = snip.Length_mm * snip.Width_mm;
					snip.list = i + 1;
					snip.nlist = -1;
					snip.Amount = 1;
					snip.X = (p + cSnip.CRD.X) / 10;
					snip.Y = (num + cSnip.CRD.Y) / 10;
					o.NSnips.Add(snip);
				}
				num = num + B + Ss[i].Lines[index].W;
				Ss[i].Lines.RemoveAt(index);
			}
			CSnip remain = Ss[i].Remain;
			Snip snip2 = new Snip();
			snip2.Length_mm = remain.L / 10;
			snip2.Width_mm = remain.W / 10;
			snip2.onList = true;
			snip2.Sq = snip2.Length_mm * snip2.Width_mm;
			snip2.list = i + 1;
			snip2.nlist = -1;
			snip2.Amount = 1;
			snip2.X = p / 10;
			snip2.Y = num / 10;
			o.NSnips.Add(snip2);
			o.SheetCount++;
		}
	}

	private void SET_ON_Parts_in_Line(List<CPart> parts, CLine line)
	{
		for (int i = 0; i < line.PartIDs.Count; i++)
		{
			if (line.PartIDs[i] < -1)
			{
				parts[line.PartIDs[i] * -1 - 2].Plased++;
			}
			else
			{
				parts[line.PartIDs[i]].Plased++;
			}
		}
	}

	private void SET_OFF_Parts_in_Line(List<CPart> parts, CLine line)
	{
		for (int i = 0; i < line.PartIDs.Count; i++)
		{
			if (line.PartIDs[i] < -1)
			{
				parts[line.PartIDs[i] * -1 - 2].Plased--;
			}
			else
			{
				parts[line.PartIDs[i]].Plased--;
			}
		}
	}

	private void Get_ID_LD_WD(List<CPart> parts, int id, out int ID, out int LD, out int WD)
	{
		if (id > -1)
		{
			ID = id;
			LD = parts[id].L;
			WD = parts[id].W;
		}
		else if (id < -1)
		{
			ID = id * -1 - 2;
			LD = parts[ID].W;
			WD = parts[ID].L;
		}
		else
		{
			ID = -1;
			LD = -1;
			WD = -1;
		}
	}

	private List<int> Get_Parts_with_FixWidth(List<CPart> parts, int W, int L, out int Min_L, out int Total_Length)
	{
		double totalMilliseconds = DateTime.Now.TimeOfDay.TotalMilliseconds;
		List<int> list = new List<int>();
		Min_L = L;
		Total_Length = 0;
		for (int i = 0; i < parts.Count; i++)
		{
			CPart cPart = parts[i];
			if (cPart.Qty <= cPart.Plased)
			{
				continue;
			}
			if (cPart.W == W)
			{
				for (int j = 0; j < cPart.Qty - cPart.Plased; j++)
				{
					list.Add(i);
					Total_Length += cPart.L;
				}
				if (Min_L > cPart.L)
				{
					Min_L = cPart.L;
				}
			}
			else if (cPart.Turn && cPart.L == W)
			{
				for (int k = 0; k < cPart.Qty - cPart.Plased; k++)
				{
					list.Add(i * -1 - 2);
					Total_Length += cPart.W;
				}
				if (Min_L > cPart.W)
				{
					Min_L = cPart.W;
				}
			}
			if (list.Count > THE_SAME_PARTS_LIMIT)
			{
				i = parts.Count;
			}
		}
		CT.T_Get_Parts_with_FixWidth += (DateTime.Now.TimeOfDay.TotalMilliseconds - totalMilliseconds) / 1000.0;
		CT.C_Get_Parts_with_FixWidth++;
		return list;
	}

	private List<int> GetStartParts_for_Line_LENGTH_CUT(List<CPart> parts, List<int> FixWidth, int LineLength, int Minimal_L)
	{
		double totalMilliseconds = DateTime.Now.TimeOfDay.TotalMilliseconds;
		List<int> list = new List<int>();
		int num = LineLength;
		for (int i = 0; i < FixWidth.Count; i++)
		{
			Get_ID_LD_WD(parts, FixWidth[i], out var ID, out var LD, out var _);
			_ = parts[ID];
			if (num >= LD)
			{
				num = num - LD - B;
				list.Add(FixWidth[i]);
				FixWidth.RemoveAt(i);
				i--;
				if (num < Minimal_L)
				{
					i = FixWidth.Count;
				}
			}
		}
		bool flag = false;
		int num2 = 0;
		int num3 = 0;
		while (!flag && num2 < PARTS_SORT_LIMIT)
		{
			num2++;
			num3 = 0;
			bool check = false;
			int num4 = -1;
			int num5 = -1;
			int num6 = 0;
			int num7 = 0;
			int[] array = null;
			for (int j = 0; j < list.Count - 1; j++)
			{
				for (int k = j + 1; k < list.Count; k++)
				{
					FixWidth.Add(list[j]);
					FixWidth.Add(list[k]);
					Get_ID_LD_WD(parts, list[j], out var _, out var LD2, out var _);
					Get_ID_LD_WD(parts, list[k], out var _, out var LD3, out var _);
					int wO = num + B + LD2 + B + LD3;
					int[] array2 = Find_Zamena_PARTS_LENGTH_CUT(FixWidth, parts, wO, LD2 + LD3, Minimal_L, out check);
					if (check)
					{
						int num8 = 0;
						for (int l = 0; l < array2.Length; l++)
						{
							if (array2[l] != -1)
							{
								Get_ID_LD_WD(parts, FixWidth[array2[l]], out var _, out var LD4, out var _);
								num8 = num8 + B + LD4;
							}
						}
						if (num8 > num3)
						{
							num4 = j;
							num5 = k;
							num6 = LD2;
							num7 = LD3;
							array = array2;
							num3 = num8;
						}
					}
					FixWidth.RemoveAt(FixWidth.Count - 1);
					FixWidth.RemoveAt(FixWidth.Count - 1);
				}
			}
			if (num4 != -1 && num5 != -1)
			{
				FixWidth.Add(list[num4]);
				FixWidth.Add(list[num5]);
				list.RemoveAt(num4);
				list.RemoveAt(num5 - 1);
				num = num + B + num6 + B + num7;
				for (int m = 0; m < array.Length; m++)
				{
					if (array[m] != -1)
					{
						list.Add(FixWidth[array[m]]);
						Get_ID_LD_WD(parts, FixWidth[array[m]], out var _, out var LD5, out var _);
						num = num - B - LD5;
					}
				}
				int num9 = 0;
				for (int n = 0; n < array.Length; n++)
				{
					if (array[n] != -1)
					{
						FixWidth.RemoveAt(array[n] - num9);
						num9++;
					}
				}
			}
			else
			{
				flag = true;
			}
		}
		for (int num10 = 0; num10 < list.Count - 1; num10++)
		{
			for (int num11 = num10 + 1; num11 < list.Count; num11++)
			{
				Get_ID_LD_WD(parts, list[num10], out var _, out var LD6, out var _);
				Get_ID_LD_WD(parts, list[num11], out var _, out var LD7, out var _);
				if (LD7 > LD6)
				{
					int value = list[num10];
					list[num10] = list[num11];
					list[num11] = value;
				}
			}
		}
		CT.T_GetStartParts_for_Line_LENGTH_CUT += (DateTime.Now.TimeOfDay.TotalMilliseconds - totalMilliseconds) / 1000.0;
		CT.C_GetStartParts_for_Line_LENGTH_CUT++;
		return list;
	}
}
