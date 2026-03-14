using System;
using System.Collections.Generic;

public class Width2
{
	private int THE_SAME_PARTS_LIMIT = 25;

	private int LINES_LIMIT = 200;

	private int LINES_SORT_ITERS_LIMIT = 4;

	private int PARTS_SORT_LIMIT = 2;

	public double TIME_GET_LINES_LIMIT = 1.0;

	private int minL;

	private int minW;

	private int P;

	private int B;

	private int L_L;

	private int L_W;

	private int PartsCount;

	private int PartsCutted;

	private CutTimeResults CR = new CutTimeResults();

	private List<CPart> CP_T_T_T_T;

	private List<CPart> CP_T_T_T_F;

	private List<CPart> CP_T_T_F_T;

	private List<CPart> CP_T_T_F_F;

	private List<CPart> CP_T_F_T_T;

	private List<CPart> CP_T_F_T_F;

	private List<CPart> CP_T_F_F_T;

	private List<CPart> CP_T_F_F_F;

	private List<CPart> CP_F_T_T_T;

	private List<CPart> CP_F_T_T_F;

	private List<CPart> CP_F_T_F_T;

	private List<CPart> CP_F_T_F_F;

	private List<CPart> CP_F_F_T_T;

	private List<CPart> CP_F_F_T_F;

	private List<CPart> CP_F_F_F_T;

	private List<CPart> CP_F_F_F_F;

	private List<CSheet> SHEETS;

	private List<CSheet> SH_T_T_T_T;

	private List<CSheet> SH_T_T_T_F;

	private List<CSheet> SH_T_T_F_T;

	private List<CSheet> SH_T_T_F_F;

	private List<CSheet> SH_T_F_T_T;

	private List<CSheet> SH_T_F_T_F;

	private List<CSheet> SH_T_F_F_T;

	private List<CSheet> SH_T_F_F_F;

	private List<CSheet> SH_F_T_T_T;

	private List<CSheet> SH_F_T_T_F;

	private List<CSheet> SH_F_T_F_T;

	private List<CSheet> SH_F_T_F_F;

	private List<CSheet> SH_F_F_T_T;

	private List<CSheet> SH_F_F_T_F;

	private List<CSheet> SH_F_F_F_T;

	private List<CSheet> SH_F_F_F_F;

	private LW16 T_T_T_T;

	private LW16 T_T_T_F;

	private LW16 T_T_F_T;

	private LW16 T_T_F_F;

	private LW16 T_F_T_T;

	private LW16 T_F_T_F;

	private LW16 T_F_F_T;

	private LW16 T_F_F_F;

	private LW16 F_T_T_T;

	private LW16 F_T_T_F;

	private LW16 F_T_F_T;

	private LW16 F_T_F_F;

	private LW16 F_F_T_T;

	private LW16 F_F_T_F;

	private LW16 F_F_F_T;

	private LW16 F_F_F_F;

	private double PSQ_T_T_T_T;

	private double PSQ_T_T_T_F;

	private double PSQ_T_T_F_T;

	private double PSQ_T_T_F_F;

	private double PSQ_T_F_T_T;

	private double PSQ_T_F_T_F;

	private double PSQ_T_F_F_T;

	private double PSQ_T_F_F_F;

	private double PSQ_F_T_T_T;

	private double PSQ_F_T_T_F;

	private double PSQ_F_T_F_T;

	private double PSQ_F_T_F_F;

	private double PSQ_F_F_T_T;

	private double PSQ_F_F_T_F;

	private double PSQ_F_F_F_T;

	private double PSQ_F_F_F_F;

	private double PartsSq;

	private double ListSQ;

	public double GetCPartsSq(List<CPart> parts)
	{
		double num = 0.0;
		for (int i = 0; i < parts.Count; i++)
		{
			num += parts[i].Sq * (double)parts[i].Qty;
		}
		return num;
	}

	public void StartCutting(Order order)
	{
		bool flag = true;
		bool flag2 = false;
		Utils.ClearCuttingInfo(order);
		SHEETS = new List<CSheet>();
		SH_T_T_T_T = new List<CSheet>();
		SH_T_T_T_F = new List<CSheet>();
		SH_T_T_F_T = new List<CSheet>();
		SH_T_T_F_F = new List<CSheet>();
		SH_T_F_T_T = new List<CSheet>();
		SH_T_F_T_F = new List<CSheet>();
		SH_T_F_F_T = new List<CSheet>();
		SH_T_F_F_F = new List<CSheet>();
		SH_F_T_T_T = new List<CSheet>();
		SH_F_T_T_F = new List<CSheet>();
		SH_F_T_F_T = new List<CSheet>();
		SH_F_T_F_F = new List<CSheet>();
		SH_F_F_T_T = new List<CSheet>();
		SH_F_F_T_F = new List<CSheet>();
		SH_F_F_F_T = new List<CSheet>();
		SH_F_F_F_F = new List<CSheet>();
		T_T_T_T = new LW16(flag, flag, flag, flag);
		T_T_T_F = new LW16(flag, flag, flag, flag2);
		T_T_F_T = new LW16(flag, flag, flag2, flag);
		T_T_F_F = new LW16(flag, flag, flag2, flag2);
		T_F_T_T = new LW16(flag, flag2, flag, flag);
		T_F_T_F = new LW16(flag, flag2, flag, flag2);
		T_F_F_T = new LW16(flag, flag2, flag2, flag);
		T_F_F_F = new LW16(flag, flag2, flag2, flag2);
		F_T_T_T = new LW16(flag2, flag, flag, flag);
		F_T_T_F = new LW16(flag2, flag, flag, flag2);
		F_T_F_T = new LW16(flag2, flag, flag2, flag);
		F_T_F_F = new LW16(flag2, flag, flag2, flag2);
		F_F_T_T = new LW16(flag2, flag2, flag, flag);
		F_F_T_F = new LW16(flag2, flag2, flag, flag2);
		F_F_F_T = new LW16(flag2, flag2, flag2, flag);
		F_F_F_F = new LW16(flag2, flag2, flag2, flag2);
		bool flag3 = false;
		int num = 0;
		PartsSq = GetCPartsSq(CP_T_T_T_T);
		while (!flag3)
		{
			num++;
			flag3 = true;
		}
		Write_Sheets_to_Order_WIDTH_CUT(order, SH_T_F_T_T, CP_T_F_T_T);
	}

	public CSheet GetCSheet_WIDTH_CUT(List<CPart> parts, int ListLength, int ListWidth, int Blade, int Padding, bool DoublePadding, LW16 PARAMS, double PSQ, double PPSQ, out double PPSQ_OUT)
	{
		L_L = ListLength;
		L_W = ListWidth;
		P = Padding;
		B = Blade;
		CSheet cSheet = new CSheet();
		cSheet.Alg = 2;
		cSheet.Lines = new List<CLine>();
		cSheet.Lines_index = new List<int>();
		cSheet.L = ListLength;
		cSheet.W = ListWidth;
		int num = Padding;
		if (DoublePadding)
		{
			num *= 2;
		}
		int num2 = cSheet.L - num;
		int num3 = cSheet.W - num;
		List<CLine> cLines_WIDTH_CUT = GetCLines_WIDTH_CUT(parts, num2, num3, PARAMS);
		int num4 = num2;
		for (int i = 0; i < cLines_WIDTH_CUT.Count; i++)
		{
			if (num4 > cLines_WIDTH_CUT[i].L)
			{
				num4 = cLines_WIDTH_CUT[i].L;
			}
		}
		int num5 = num2;
		for (int j = 0; j < cLines_WIDTH_CUT.Count; j++)
		{
			if (num5 >= cLines_WIDTH_CUT[j].L)
			{
				num5 = num5 - cLines_WIDTH_CUT[j].L - B;
				cSheet.Lines.Add(cLines_WIDTH_CUT[j]);
				cSheet.Lines_index.Add(j);
				cLines_WIDTH_CUT[j].onSheet = true;
				if (num4 >= num5)
				{
					j = cLines_WIDTH_CUT.Count;
				}
			}
		}
		bool flag = false;
		int num6 = 0;
		while (!flag && num6 < LINES_SORT_ITERS_LIMIT)
		{
			num6++;
			bool check = false;
			int num7 = -1;
			int num8 = -1;
			int[] array = null;
			double num9 = 0.0;
			for (int k = 0; k < cSheet.Lines.Count - 1; k++)
			{
				for (int l = k + 1; l < cSheet.Lines.Count; l++)
				{
					cSheet.Lines[k].onSheet = false;
					cSheet.Lines[l].onSheet = false;
					int wO = num5 + B + cSheet.Lines[k].L + B + cSheet.Lines[l].L;
					int[] array2 = Find_Zamena_Lines_WIDTH_CUT(cLines_WIDTH_CUT, wO, num4, out check);
					if (cSheet.Lines_index[k] != array2[0] || cSheet.Lines_index[l] != array2[1] || array2[2] != -1)
					{
						int num10 = cSheet.Lines[k].L + cSheet.Lines[l].L;
						double num11 = cSheet.Lines[k].Parts_Sq + cSheet.Lines[l].Parts_Sq;
						int num12 = 0;
						double num13 = 0.0;
						for (int m = 0; m < 3; m++)
						{
							if (array2[m] != -1)
							{
								num13 += cLines_WIDTH_CUT[array2[m]].Parts_Sq;
								num12 += cLines_WIDTH_CUT[array2[m]].L;
							}
						}
						if (num12 >= num10 && (long)(num13 - num11) >= 0 && ((long)(num13 - num9) > 0 || ((long)(num13 - num9) == 0L && num12 - cSheet.Lines[num7].L - cSheet.Lines[num8].L > 0)))
						{
							num7 = k;
							num8 = l;
							array = array2;
							num9 = num13;
						}
					}
					cSheet.Lines[k].onSheet = true;
					cSheet.Lines[l].onSheet = true;
				}
			}
			if (num7 != -1 && num8 != -1)
			{
				num5 = num5 + B + cSheet.Lines[num7].L + B + cSheet.Lines[num8].L;
				cSheet.Lines[num7].onSheet = false;
				cSheet.Lines[num8].onSheet = false;
				cSheet.Lines.Remove(cSheet.Lines[num7]);
				cSheet.Lines.Remove(cSheet.Lines[num8 - 1]);
				cSheet.Lines_index.Remove(num7);
				cSheet.Lines_index.Remove(num8 - 1);
				for (int n = 0; n < 3; n++)
				{
					if (array[n] != -1)
					{
						cSheet.Lines.Add(cLines_WIDTH_CUT[array[n]]);
						cSheet.Lines_index.Add(array[n]);
						cLines_WIDTH_CUT[array[n]].onSheet = true;
						num5 = num5 - B - cLines_WIDTH_CUT[array[n]].L;
					}
				}
			}
			else
			{
				flag = true;
			}
		}
		cSheet.Remain = new CSnip();
		cSheet.Remain.L = num5;
		cSheet.Remain.W = num3;
		for (int num14 = 0; num14 < cSheet.Lines.Count - 1; num14++)
		{
			for (int num15 = num14 + 1; num15 < cSheet.Lines.Count; num15++)
			{
				if (cSheet.Lines[num15].L > cSheet.Lines[num14].L)
				{
					int value = cSheet.Lines_index[num14];
					cSheet.Lines_index[num14] = cSheet.Lines_index[num15];
					cSheet.Lines_index[num15] = value;
					CLine value2 = cSheet.Lines[num14];
					cSheet.Lines[num14] = cSheet.Lines[num15];
					cSheet.Lines[num15] = value2;
				}
			}
		}
		for (int num16 = 0; num16 < cLines_WIDTH_CUT.Count; num16++)
		{
			if (!cLines_WIDTH_CUT[num16].onSheet)
			{
				SET_OFF_Parts_in_Line(parts, cLines_WIDTH_CUT[num16]);
			}
		}
		cSheet.Parts_Sq = 0.0;
		for (int num17 = cSheet.Lines.Count - 1; num17 >= 0; num17--)
		{
			Continue_Line_WIDTH_CUT(cSheet.Lines[num17], parts, PARAMS);
			cSheet.Parts_Sq += cSheet.Lines[num17].Parts_Sq;
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
				int num18 = Find_LENGTH_part(parts, cSheet.Remain.L, cSheet.Remain.W, Max_L: true);
				Get_ID_LD_WD(parts, num18, out var _, out var LD, out var _);
				cLine.W = cSheet.Remain.W;
				cLine.L = LD;
				cSnip.CRD = new Crd();
				cSnip.CRD.X = 0;
				cSnip.CRD.Y = 0;
				cSnip.L = cLine.L;
				cSnip.W = cLine.W;
				cLine.Snips.Add(cSnip);
				int io = FindSmallSnip(cLine.Snips, parts);
				Place_Part_to_Line(cLine, parts, num18, io, _rez: true);
				Continue_Line_WIDTH_CUT(cLine, parts, PARAMS);
				cSheet.Remain.L = cSheet.Remain.L - B - cLine.L;
				cSheet.Lines.Add(cLine);
				cSheet.Parts_Sq += cLine.Parts_Sq;
			}
			else
			{
				flag = true;
			}
		}
		PPSQ_OUT = PPSQ + cSheet.Parts_Sq;
		return cSheet;
	}

	private void Continue_Line_WIDTH_CUT(CLine LINE, List<CPart> parts, LW16 PARAMS)
	{
		int num = -1;
		for (int num2 = FindSmallSnip(LINE.Snips, parts); num2 >= 0; num2 = FindSmallSnip(LINE.Snips, parts))
		{
			int l = LINE.Snips[num2].L;
			int w = LINE.Snips[num2].W;
			num = ((!PARAMS.MAX_SQ) ? Find_LENGTH_part(parts, l, w, Max_L: true) : FindMaxSqPart(parts, l, w));
			if (num != -1)
			{
				if (PARAMS.OPTI_ON)
				{
					int[] array = Check_part_for_last_in_Line(parts, l, w, num);
					if (num != array[0] && array[0] != -1)
					{
						Place_2_Parts_to_Line(LINE, parts, array, num2);
					}
					else
					{
						Place_Part_to_Line(LINE, parts, num, num2, _rez: true);
					}
				}
				else
				{
					Place_Part_to_Line(LINE, parts, num, num2, _rez: true);
				}
			}
		}
	}

	private List<CLine> GetCLines_WIDTH_CUT(List<CPart> parts, int LL, int LW, LW16 PARAMS)
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
			CLine cLine2 = null;
			num = ((!PARAMS.MAX_SQ) ? Find_LENGTH_part(parts, num3, num4, Max_L: false) : FindMaxSqPart(parts, LL, LW));
			if (num != -1)
			{
				cLine = MakeLine_WIDTH_CUT(parts, num, num3, num4, PARAMS, out var _);
				if (PARAMS.TURN_ON)
				{
					int index = num;
					if (num < -1)
					{
						index = num * -1 - 2;
					}
					if (parts[index].Turn && ((num < -1 && num3 >= parts[index].L && num4 >= parts[index].W) || (num > -1 && num3 >= parts[index].W && num4 >= parts[index].L)))
					{
						cLine2 = MakeLine_WIDTH_CUT(parts, num * -1 - 2, num3, num4, PARAMS, out var _);
					}
					if (cLine != null && cLine2 != null && (int)((cLine.Filling - cLine2.Filling) * 100f) < 0)
					{
						cLine = cLine2;
					}
				}
			}
			if (cLine != null)
			{
				SET_ON_Parts_in_Line(parts, cLine);
				list.Add(cLine);
				num3 = num3 - B - cLine.L;
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
		return list;
	}

	private CLine MakeLine_WIDTH_CUT(List<CPart> parts, int startPart, int LineLength, int LineWidth, LW16 PARAMS, out CLine PreCut)
	{
		CLine cLine = new CLine();
		CSnip cSnip = new CSnip();
		cLine.Snips = new List<CSnip>();
		cLine.PartIDs = new List<int>();
		cLine.Parts_Crds = new List<Crd>();
		Get_ID_LD_WD(parts, startPart, out var _, out var LD, out var _);
		bool rez = true;
		cLine.W = LineWidth;
		cLine.L = LD;
		cSnip.CRD = new Crd();
		cSnip.CRD.X = 0;
		cSnip.CRD.Y = 0;
		cSnip.L = cLine.L;
		cSnip.W = cLine.W;
		cLine.Snips.Add(cSnip);
		int io = 0;
		if (PARAMS.SAME_MAX)
		{
			int Min_W;
			int Total_Length;
			List<int> fixLength = Get_Parts_with_FixLength(parts, LD, cLine.W, PARAMS.TURN_ON, out Min_W, out Total_Length);
			List<int> startParts_for_Line_WIDTH_CUT = GetStartParts_for_Line_WIDTH_CUT(parts, fixLength, cLine.W, Min_W);
			io = FindSmallSnip(cLine.Snips, parts);
			for (int i = 0; i < startParts_for_Line_WIDTH_CUT.Count; i++)
			{
				Place_Part_to_Line(cLine, parts, startParts_for_Line_WIDTH_CUT[i], io, rez);
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
					num = Find_THE_SAME_LENGTH_part(parts, l, w, PARAMS.TURN_ON);
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
		Continue_Line_WIDTH_CUT(cLine, parts, PARAMS);
		SET_OFF_Parts_in_Line(parts, cLine);
		return cLine;
	}

	private CLine CopyLine_WITHOUT_MARKS(CLine LINE)
	{
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
		return cLine;
	}

	private int[] Find_2_Lines(List<CLine> Lines, int size, double SQ, bool REZ, out bool Check, out double SQ_zamena)
	{
		int[] array = new int[2] { -1, -1 };
		Check = false;
		int num = -1;
		int num2 = -1;
		SQ_zamena = 0.0;
		int num3 = 0;
		int num4 = 0;
		double num5 = SQ;
		for (int i = 0; i < Lines.Count; i++)
		{
			if (Lines[i].onSheet)
			{
				continue;
			}
			num3 = (REZ ? Lines[i].L : Lines[i].W);
			if (size < num3)
			{
				continue;
			}
			for (int j = 0; j < Lines.Count; j++)
			{
				if (i != j && !Lines[j].onSheet)
				{
					num4 = (REZ ? Lines[j].L : Lines[j].W);
					if (size - num3 - B - num4 >= 0 && (long)(Lines[i].Parts_Sq + Lines[j].Parts_Sq - num5) > 0)
					{
						num5 = Lines[i].Parts_Sq + Lines[j].Parts_Sq;
						num = i;
						num2 = j;
						Check = true;
					}
				}
			}
		}
		array[0] = num;
		array[1] = num2;
		if (Check)
		{
			SQ_zamena = Lines[num].Parts_Sq + Lines[num2].Parts_Sq;
		}
		return array;
	}

	private bool FastFindFirstPart(List<CPart> parts, int LO, int WO)
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

	private int Find_LENGTH_part(List<CPart> parts, int LO, int WO, bool Max_L)
	{
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
					if (parts[i].L > num)
					{
						num2 = parts[i].Sq;
						num = parts[i].L;
						result = i;
					}
					else if (parts[i].L == num && (long)(parts[i].Sq - num2) > 0)
					{
						num2 = parts[i].Sq;
						num = parts[i].L;
						result = i;
					}
				}
			}
			else if (parts[i].Turn)
			{
				int num3 = 0;
				if (LO >= parts[i].L && WO >= parts[i].W && LO >= parts[i].W && WO >= parts[i].L)
				{
					num3 = (Max_L ? ((parts[i].L < parts[i].W) ? parts[i].W : parts[i].L) : ((parts[i].L < parts[i].W) ? parts[i].L : parts[i].W));
				}
				else if (LO >= parts[i].L && WO >= parts[i].W && (LO < parts[i].W || WO < parts[i].L))
				{
					num3 = parts[i].L;
				}
				else if ((LO < parts[i].L || WO < parts[i].W) && LO >= parts[i].W && WO >= parts[i].L)
				{
					num3 = parts[i].W;
				}
				if (num3 > num)
				{
					num2 = parts[i].Sq;
					num = num3;
					result = ((parts[i].L != num3) ? (-1 * i - 2) : i);
				}
				else if (num3 == num && (long)(parts[i].Sq - num2) > 0)
				{
					num2 = parts[i].Sq;
					num = num3;
					result = ((parts[i].L != num3) ? (-1 * i - 2) : i);
				}
			}
		}
		return result;
	}

	private int Find_THE_SAME_LENGTH_part(List<CPart> parts, int LO, int WO, bool TURN_ON)
	{
		int result = -1;
		double num = 0.0;
		for (int i = 0; i < parts.Count; i++)
		{
			if (parts[i].Plased >= parts[i].Qty)
			{
				continue;
			}
			if (!TURN_ON)
			{
				if (!parts[i].Turn)
				{
					if (LO == parts[i].L && WO >= parts[i].W && (long)(parts[i].Sq - num) > 0)
					{
						num = parts[i].Sq;
						result = i;
					}
				}
				else
				{
					if (!parts[i].Turn)
					{
						continue;
					}
					if (parts[i].L > parts[i].W && LO == parts[i].W && WO >= parts[i].L)
					{
						if ((long)(parts[i].Sq - num) > 0)
						{
							num = parts[i].Sq;
							result = i * -1 - 2;
						}
					}
					else if (parts[i].W > parts[i].L && LO == parts[i].L && WO >= parts[i].W && (long)(parts[i].Sq - num) > 0)
					{
						num = parts[i].Sq;
						result = i;
					}
				}
			}
			else if (LO == parts[i].L && WO >= parts[i].W)
			{
				if ((long)(parts[i].Sq - num) > 0)
				{
					num = parts[i].Sq;
					result = i;
				}
			}
			else if (parts[i].Turn && LO == parts[i].W && WO >= parts[i].L && (long)(parts[i].Sq - num) > 0)
			{
				num = parts[i].Sq;
				result = i * -1 - 2;
			}
		}
		return result;
	}

	private void Place_Part_to_Line(CLine line, List<CPart> parts, int part_id, int io, bool _rez)
	{
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
	}

	private void Place_2_Parts_to_Line(CLine line, List<CPart> parts, int[] _2parts, int io)
	{
		int l = line.Snips[io].L;
		int w = line.Snips[io].W;
		int x = line.Snips[io].CRD.X;
		int y = line.Snips[io].CRD.Y;
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
				num27 = (((long)((num14 - num13) * 10.0) == 0L) ? ((l < w) ? (num11 * -1) : num12) : (((long)(num14 - num13) >= 0) ? (num11 * -1) : num12));
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
			case 0:
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
				num28 = (((long)((sqPartsForSnips7 - sqPartsForSnips8) * 100.0) >= 0) ? 1 : 2);
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
	}

	private int[] Check_part_for_last_in_Line(List<CPart> parts, int LO, int WO, int id)
	{
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
		return num;
	}

	private int FindMaxSqPart(List<CPart> parts, int LO, int WO)
	{
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
		return result;
	}

	private int FindMaxSqPart(List<CPart> parts, int LO, int WO, int krome)
	{
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
		return result;
	}

	private int FindSmallSnip(List<CSnip> snips, List<CPart> parts)
	{
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
		if ((long)(num5 - num6) > 0)
		{
			return array;
		}
		return array2;
	}

	private int[] Find_Zamena_Lines_WIDTH_CUT(List<CLine> Lines, int WO, int Minimal_L, out bool check)
	{
		int[] array = new int[3] { -1, -1, -1 };
		check = false;
		int num = 0;
		for (int i = 0; i < Lines.Count; i++)
		{
			int l = Lines[i].L;
			if (Lines[i].onSheet || WO < l)
			{
				continue;
			}
			if (l > num)
			{
				array[0] = i;
				array[1] = -1;
				array[2] = -1;
				num = l;
				check = true;
			}
			else if (l == num)
			{
				double num2 = 0.0;
				for (int j = 0; j < 3; j++)
				{
					if (array[j] != -1)
					{
						num2 += Lines[array[j]].Parts_Sq;
					}
				}
				if ((long)(Lines[i].Parts_Sq - num2) >= 0)
				{
					array[0] = i;
					array[1] = -1;
					array[2] = -1;
					num = l;
					check = true;
				}
			}
			if (WO - l - B - Minimal_L < 0)
			{
				continue;
			}
			for (int k = i + 1; k < Lines.Count; k++)
			{
				int l2 = Lines[k].L;
				if (Lines[k].onSheet || WO < l2)
				{
					continue;
				}
				if (WO - l - B - l2 >= 0)
				{
					if (l + l2 - num > 0)
					{
						array[0] = i;
						array[1] = k;
						array[2] = -1;
						num = l + l2;
						check = true;
					}
					else if (l + l2 - num == 0)
					{
						double num3 = 0.0;
						for (int m = 0; m < 3; m++)
						{
							if (array[m] != -1)
							{
								num3 += Lines[array[m]].Parts_Sq;
							}
						}
						if ((long)(Lines[i].Parts_Sq + Lines[k].Parts_Sq - num3) >= 0)
						{
							array[0] = i;
							array[1] = k;
							array[2] = -1;
							num = l + l2;
							check = true;
						}
					}
				}
				if (WO - l - B - l2 - B - Minimal_L < 0)
				{
					continue;
				}
				for (int n = k + 1; n < Lines.Count; n++)
				{
					int l3 = Lines[n].L;
					if (Lines[n].onSheet || WO < l3 || WO - l - B - l2 - B - l3 < 0)
					{
						continue;
					}
					if (l + l2 + l3 - num > 0)
					{
						array[0] = i;
						array[1] = k;
						array[2] = n;
						num = l + l2 + l3;
						check = true;
					}
					else
					{
						if (l + l2 + l3 - num != 0)
						{
							continue;
						}
						double num4 = 0.0;
						for (int num5 = 0; num5 < 3; num5++)
						{
							if (array[num5] != -1)
							{
								num4 += Lines[array[num5]].Parts_Sq;
							}
						}
						if ((long)(Lines[i].Parts_Sq + Lines[k].Parts_Sq + Lines[n].Parts_Sq - num4) >= 0)
						{
							array[0] = i;
							array[1] = k;
							array[2] = n;
							num = l + l2 + l3;
							check = true;
						}
					}
				}
			}
		}
		return array;
	}

	private int[] Find_Zamena_PARTS_WIDTH_CUT(List<int> Fix, List<CPart> parts, int WO, int max_W, int Minimal_W, out bool check)
	{
		int[] array = new int[3] { -1, -1, -1 };
		check = false;
		for (int i = 0; i < Fix.Count; i++)
		{
			int num = ((Fix[i] <= -1) ? parts[Fix[i] * -1 - 2].L : parts[Fix[i]].W);
			if (WO < num)
			{
				continue;
			}
			if (num > max_W)
			{
				array[0] = i;
				array[1] = -1;
				array[2] = -1;
				max_W = num;
				check = true;
			}
			if (WO - num - B - Minimal_W < 0)
			{
				continue;
			}
			for (int j = i + 1; j < Fix.Count; j++)
			{
				int num2 = ((Fix[j] <= -1) ? parts[Fix[j] * -1 - 2].L : parts[Fix[j]].W);
				if (WO - num - B - num2 >= 0 && num + num2 - max_W > 0)
				{
					array[0] = i;
					array[1] = j;
					array[2] = -1;
					max_W = num + num2;
					check = true;
				}
				if (WO - num - B - num2 - B - Minimal_W < 0)
				{
					continue;
				}
				for (int k = j + 1; k < Fix.Count; k++)
				{
					int num3 = ((Fix[k] <= -1) ? parts[Fix[k] * -1 - 2].L : parts[Fix[k]].W);
					if (WO - num - B - num2 - B - num3 >= 0 && num + num2 + num3 - max_W > 0)
					{
						array[0] = i;
						array[1] = j;
						array[2] = k;
						max_W = num + num2 + num3;
						check = true;
					}
				}
			}
		}
		return array;
	}

	private void Write_Sheets_to_Order_WIDTH_CUT(Order o, List<CSheet> Ss, List<CPart> CPARTS)
	{
		for (int i = 0; i < Ss.Count; i++)
		{
			int num = P;
			int p = P;
			while (Ss[i].Lines.Count > 0)
			{
				int num2 = 0;
				int index = -1;
				for (int j = 0; j < Ss[i].Lines.Count; j++)
				{
					if (Ss[i].Lines[j].L > num2 || (Ss[i].Lines[j].L == num2 && (long)(Ss[i].Lines[j].Parts_Sq - Ss[i].Lines[index].Parts_Sq) > 0))
					{
						index = j;
						num2 = Ss[i].Lines[j].L;
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
					part.Coords[nPlased].X = num + crd.X;
					part.Coords[nPlased].Y = p + crd.Y;
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
					snip.Length_mm = cSnip.L;
					snip.Width_mm = cSnip.W;
					snip.onList = true;
					snip.Sq = snip.Length_mm * snip.Width_mm;
					snip.list = i + 1;
					snip.nlist = -1;
					snip.Amount = 1;
					snip.X = num + cSnip.CRD.X;
					snip.Y = p + cSnip.CRD.Y;
					o.NSnips.Add(snip);
				}
				num = num + B + Ss[i].Lines[index].L;
				Ss[i].Lines.RemoveAt(index);
			}
			CSnip remain = Ss[i].Remain;
			Snip snip2 = new Snip();
			snip2.Length_mm = remain.L;
			snip2.Width_mm = remain.W;
			snip2.onList = true;
			snip2.Sq = snip2.Length_mm * snip2.Width_mm;
			snip2.list = i + 1;
			snip2.nlist = -1;
			snip2.Amount = 1;
			snip2.X = num;
			snip2.Y = p;
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

	private List<int> Get_Parts_with_FixLength(List<CPart> parts, int L, int min_in, bool TURN_ON, out int Min_W, out int Total_Length)
	{
		List<int> list = new List<int>();
		Min_W = min_in;
		Total_Length = 0;
		for (int i = 0; i < parts.Count; i++)
		{
			CPart cPart = parts[i];
			if (cPart.Qty <= cPart.Plased)
			{
				continue;
			}
			if (!TURN_ON)
			{
				int l = cPart.L;
				if (!cPart.Turn)
				{
					if (L == cPart.L)
					{
						for (int j = 0; j < cPart.Qty - cPart.Plased; j++)
						{
							list.Add(i);
							Total_Length += cPart.W;
						}
						if (Min_W > cPart.W)
						{
							Min_W = cPart.W;
						}
					}
				}
				else if (cPart.Turn)
				{
					bool flag = false;
					if (cPart.L <= cPart.W)
					{
						l = cPart.L;
						flag = false;
					}
					else
					{
						l = cPart.W;
						flag = true;
					}
					if (l == L)
					{
						for (int k = 0; k < cPart.Qty - cPart.Plased; k++)
						{
							if (flag)
							{
								list.Add(i * -1 - 2);
								Total_Length += cPart.L;
							}
							else
							{
								list.Add(i);
								Total_Length += cPart.W;
							}
						}
						if (flag)
						{
							if (Min_W > cPart.L)
							{
								Min_W = cPart.L;
							}
							else if (Min_W > cPart.W)
							{
								Min_W = cPart.W;
							}
						}
					}
				}
			}
			else if (cPart.L == L)
			{
				for (int m = 0; m < cPart.Qty - cPart.Plased; m++)
				{
					list.Add(i);
					Total_Length += cPart.W;
				}
				if (Min_W > cPart.W)
				{
					Min_W = cPart.W;
				}
			}
			else if (cPart.Turn && cPart.W == L)
			{
				for (int n = 0; n < cPart.Qty - cPart.Plased; n++)
				{
					list.Add(i * -1 - 2);
					Total_Length += cPart.L;
				}
				if (Min_W > cPart.L)
				{
					Min_W = cPart.L;
				}
			}
			if (list.Count > THE_SAME_PARTS_LIMIT)
			{
				i = parts.Count;
			}
		}
		return list;
	}

	private List<int> GetStartParts_for_Line_WIDTH_CUT(List<CPart> parts, List<int> FixLength, int LineWidth, int Minimal_W)
	{
		List<int> list = new List<int>();
		int num = LineWidth;
		for (int i = 0; i < FixLength.Count; i++)
		{
			Get_ID_LD_WD(parts, FixLength[i], out var ID, out var _, out var WD);
			_ = parts[ID];
			if (num >= WD)
			{
				num = num - WD - B;
				list.Add(FixLength[i]);
				FixLength.RemoveAt(i);
				i--;
				if (num < Minimal_W)
				{
					i = FixLength.Count;
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
					FixLength.Add(list[j]);
					FixLength.Add(list[k]);
					int num8 = ((list[j] <= -1) ? parts[list[j] * -1 - 2].L : parts[list[j]].W);
					int num9 = ((list[k] <= -1) ? parts[list[k] * -1 - 2].L : parts[list[k]].W);
					int wO = num + B + num8 + B + num9;
					int[] array2 = Find_Zamena_PARTS_WIDTH_CUT(FixLength, parts, wO, num8 + num9, Minimal_W, out check);
					if (check)
					{
						int num10 = 0;
						for (int l = 0; l < array2.Length; l++)
						{
							if (array2[l] != -1)
							{
								Get_ID_LD_WD(parts, FixLength[array2[l]], out var _, out var _, out var WD2);
								num10 = num10 + B + WD2;
							}
						}
						if (num10 > num3)
						{
							num4 = j;
							num5 = k;
							num6 = num8;
							num7 = num9;
							array = array2;
							num3 = num10;
						}
					}
					FixLength.RemoveAt(FixLength.Count - 1);
					FixLength.RemoveAt(FixLength.Count - 1);
				}
			}
			if (num4 != -1 && num5 != -1)
			{
				FixLength.Add(list[num4]);
				FixLength.Add(list[num5]);
				list.RemoveAt(num4);
				list.RemoveAt(num5 - 1);
				num = num + B + num6 + B + num7;
				for (int m = 0; m < array.Length; m++)
				{
					if (array[m] != -1)
					{
						list.Add(FixLength[array[m]]);
						Get_ID_LD_WD(parts, FixLength[array[m]], out var _, out var _, out var WD3);
						num = num - B - WD3;
					}
				}
				int num11 = 0;
				for (int n = 0; n < array.Length; n++)
				{
					if (array[n] != -1)
					{
						FixLength.RemoveAt(array[n] - num11);
						num11++;
					}
				}
			}
			else
			{
				flag = true;
			}
		}
		for (int num12 = 0; num12 < list.Count - 1; num12++)
		{
			for (int num13 = num12 + 1; num13 < list.Count; num13++)
			{
				Get_ID_LD_WD(parts, list[num12], out var _, out var _, out var WD4);
				Get_ID_LD_WD(parts, list[num13], out var _, out var _, out var WD5);
				if (WD5 > WD4)
				{
					int value = list[num12];
					list[num12] = list[num13];
					list[num13] = value;
				}
			}
		}
		return list;
	}
}
