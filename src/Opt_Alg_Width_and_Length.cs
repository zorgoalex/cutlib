using System.Collections.Generic;


public class Opt_Alg_Width_and_Length 
{
	public int THE_SAME_PARTS_LIMIT = 40;

	public int LINES_LIMIT = 200;

	public int LINES_SORT_ITERS_LIMIT = 4;

	public int PARTS_SORT_LIMIT = 2;

	public int GET_SHEET_ITER = 3;

	private int B;

	private List<CPart> Parts_OPT;

	private List<CSheet> Sheets_OPT;

	private int L_L;

	private int L_W;

	private int P;

	public void StartCutting_OPT_ONLY(Order order)
	{
		Utils.ClearCuttingInfo(order);
		Sheets_OPT = new List<CSheet>();
		bool flag = true;
		bool mAX_SQ = false;
		bool flag2 = false;
		int num = 0;
		while (!flag2)
		{
			num++;
			if (FastFindFirstPart(Parts_OPT, L_L - 2 * P, L_W - 2 * P))
			{
				CSheet cSheet = Get_Sheet_OPT_ALG_2(Parts_OPT, L_L, L_W, B, P, DoublePadding: true, flag, mAX_SQ, flag, flag, 3);
				Sheets_OPT.Add(cSheet);
				SET_ON_Parts_in_Sheet(Parts_OPT, cSheet);
			}
			else
			{
				flag2 = true;
			}
		}
		AlgUtils.Write_Sheets_to_Order(order, Sheets_OPT);
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

	public CSheet Get_Sheet_OPT_ALG_2(List<CPart> parts, int ListLength, int ListWidth, int Blade, int Padding, bool DoublePadding, bool SAME_MAX, bool MAX_SQ, bool OPTI_ON, bool TURN_ON, int ALG)
	{
		parts = Clean_CParts(parts);
		B = Blade;
		Length_Alg length_Alg = new Length_Alg();
		length_Alg.THE_SAME_PARTS_LIMIT = THE_SAME_PARTS_LIMIT;
		length_Alg.LINES_LIMIT = LINES_LIMIT;
		length_Alg.LINES_SORT_ITERS_LIMIT = LINES_SORT_ITERS_LIMIT;
		length_Alg.PARTS_SORT_LIMIT = PARTS_SORT_LIMIT;
		Width_Alg width_Alg = new Width_Alg();
		width_Alg.THE_SAME_PARTS_LIMIT = THE_SAME_PARTS_LIMIT;
		width_Alg.LINES_LIMIT = LINES_LIMIT;
		width_Alg.LINES_SORT_ITERS_LIMIT = LINES_SORT_ITERS_LIMIT;
		width_Alg.PARTS_SORT_LIMIT = PARTS_SORT_LIMIT;
		CSheet cSheet_LENGTH_CUT = length_Alg.GetCSheet_LENGTH_CUT(parts, ListLength, ListWidth, Blade, Padding, DoublePadding, Opti_ON: true, CleanParts: false);
		CSheet cSheet = Create_CSheet(ListLength, ListWidth, Blade, Padding, DoublePadding);
		for (int i = 0; i < cSheet_LENGTH_CUT.Lines.Count; i++)
		{
			Put_Line_to_Sheet(cSheet, cSheet_LENGTH_CUT.Lines[i]);
		}
		cSheet.Alg = 1;
		SET_OFF_Parts_in_Sheet(parts, cSheet);
		CSheet cSheet_WIDTH_CUT = width_Alg.GetCSheet_WIDTH_CUT(parts, ListLength, ListWidth, Blade, Padding, DoublePadding, Opti_ON: true, CleanParts: false);
		CSheet cSheet2 = Create_CSheet(ListLength, ListWidth, Blade, Padding, DoublePadding);
		for (int j = 0; j < cSheet_WIDTH_CUT.Lines.Count; j++)
		{
			Put_Line_to_Sheet(cSheet2, cSheet_WIDTH_CUT.Lines[j]);
		}
		SET_OFF_Parts_in_Sheet(parts, cSheet2);
		cSheet2.Alg = 2;
		List<CSheet> list = new List<CSheet>();
		list.Add(cSheet);
		list.Add(cSheet2);
		bool flag = false;
		int num = 0;
		while (!flag && num < GET_SHEET_ITER)
		{
			flag = true;
			num++;
			int count = list.Count;
			int num2 = 0;
			for (int k = 0; k < count; k++)
			{
				CSheet cSheet3 = list[k];
				CSheet cSheet4 = Create_CSheet(cSheet3.L, cSheet3.W, Blade, Padding, DoublePadding);
				if (cSheet3.Alg == 1)
				{
					cSheet4.Alg = 1;
				}
				else
				{
					cSheet4.Alg = 2;
				}
				if (num >= cSheet3.Lines.Count)
				{
					continue;
				}
				num2++;
				for (int l = 0; l < num; l++)
				{
					Put_Line_to_Sheet(cSheet4, cSheet3.Lines[l]);
				}
				SET_ON_Parts_in_Sheet(parts, cSheet4);
				if (FastFindFirstPart(parts, cSheet4.Remain.L, cSheet4.Remain.W))
				{
					if (cSheet4.Alg == 1)
					{
						CSheet cSheet_WIDTH_CUT2 = width_Alg.GetCSheet_WIDTH_CUT(parts, cSheet4.Remain.L, cSheet4.Remain.W, Blade, 0, DoublePadding: false, Opti_ON: true, CleanParts: false);
						cSheet4.Alg = 2;
						SET_OFF_Parts_in_Sheet(parts, cSheet_WIDTH_CUT2);
						SET_OFF_Parts_in_Sheet(parts, cSheet4);
						for (int m = 0; m < cSheet_WIDTH_CUT2.Lines.Count; m++)
						{
							Put_Line_to_Sheet(cSheet4, cSheet_WIDTH_CUT2.Lines[m]);
						}
					}
					else if (cSheet4.Alg == 2)
					{
						CSheet cSheet_LENGTH_CUT2 = length_Alg.GetCSheet_LENGTH_CUT(parts, cSheet4.Remain.L, cSheet4.Remain.W, Blade, 0, DoublePadding: false, Opti_ON: true, CleanParts: false);
						cSheet4.Alg = 1;
						SET_OFF_Parts_in_Sheet(parts, cSheet_LENGTH_CUT2);
						SET_OFF_Parts_in_Sheet(parts, cSheet4);
						for (int n = 0; n < cSheet_LENGTH_CUT2.Lines.Count; n++)
						{
							Put_Line_to_Sheet(cSheet4, cSheet_LENGTH_CUT2.Lines[n]);
						}
					}
					flag = false;
					list.Add(cSheet4);
				}
				else
				{
					SET_OFF_Parts_in_Sheet(parts, cSheet4);
				}
			}
		}
		while (list.Count != 1)
		{
			if (list[0].Parts_Sq > list[list.Count - 1].Parts_Sq)
			{
				list.RemoveAt(list.Count - 1);
			}
			else if (list[0].Parts_Sq < list[list.Count - 1].Parts_Sq)
			{
				list.RemoveAt(0);
			}
			else if (list[0].Parts_Sq == list[list.Count - 1].Parts_Sq && list[0].Remain.L * list[0].Remain.W >= list[list.Count - 1].Remain.L * list[list.Count - 1].Remain.W)
			{
				list.RemoveAt(list.Count - 1);
			}
			else
			{
				list.RemoveAt(0);
			}
		}
		SET_ON_Parts_in_Sheet(parts, list[0]);
		list[0].Alg = 3;
		return list[0];
	}

	private void Put_Line_to_Sheet(CSheet sheet, CLine line)
	{
		line.crd = new Crd();
		line.crd.X = sheet.Remain.CRD.X;
		line.crd.Y = sheet.Remain.CRD.Y;
		if (line.L == sheet.Remain.L && line.W < sheet.Remain.W)
		{
			sheet.Remain.CRD.Y = sheet.Remain.CRD.Y + line.W + B;
			sheet.Remain.W = sheet.Remain.W - line.W - B;
		}
		else if (line.L < sheet.Remain.L && line.W == sheet.Remain.W)
		{
			sheet.Remain.CRD.X = sheet.Remain.CRD.X + line.L + B;
			sheet.Remain.L = sheet.Remain.L - line.L - B;
		}
		else if (line.L == sheet.Remain.L && line.W == sheet.Remain.W)
		{
			sheet.Remain.CRD.X = sheet.Remain.CRD.X + line.L + B;
			sheet.Remain.L = sheet.Remain.L - line.L - B;
			sheet.Remain.CRD.Y = sheet.Remain.CRD.Y + line.W + B;
			sheet.Remain.W = sheet.Remain.W - line.W - B;
		}
		sheet.Lines.Add(line);
		sheet.Parts_Sq += line.Parts_Sq;
	}

	private CSheet Create_CSheet(int ListLength, int ListWidth, int Blade, int Padding, bool DoublePadding)
	{
		CSheet cSheet = new CSheet();
		cSheet.Alg = 3;
		cSheet.Lines = new List<CLine>();
		cSheet.L = ListLength;
		cSheet.W = ListWidth;
		int num = Padding;
		if (DoublePadding)
		{
			num *= 2;
		}
		cSheet.Remain = new CSnip();
		cSheet.Remain.L = cSheet.L - num;
		cSheet.Remain.W = cSheet.W - num;
		cSheet.Remain.CRD = new Crd();
		if (DoublePadding)
		{
			cSheet.Remain.CRD.X = Padding;
			cSheet.Remain.CRD.Y = Padding;
		}
		else
		{
			cSheet.Remain.CRD.X = 0;
			cSheet.Remain.CRD.Y = 0;
		}
		return cSheet;
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

	private void SET_ON_Parts_in_Sheet(List<CPart> parts, CSheet sheet)
	{
		for (int i = 0; i < sheet.Lines.Count; i++)
		{
			SET_ON_Parts_in_Line(parts, sheet.Lines[i]);
		}
	}

	private void SET_OFF_Parts_in_Sheet(List<CPart> parts, CSheet sheet)
	{
		for (int i = 0; i < sheet.Lines.Count; i++)
		{
			SET_OFF_Parts_in_Line(parts, sheet.Lines[i]);
		}
	}
}
