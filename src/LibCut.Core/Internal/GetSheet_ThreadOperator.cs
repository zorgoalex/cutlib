using System.Collections.Generic;

public class GetSheet_ThreadOperator
{
	public List<CPart> Parts;

	public CSheet Sheet;

	private LW16 PARAMS;

	private double PSQ;

	public double PPSQ;

	private int LL;

	private int LW;

	private int B;

	private int P;

	private bool DP = true;

	public int alg = 1;

	private bool l16 = true;

	public GetSheet_ThreadOperator(List<CPart> CParts, Order order, bool SAME_MAX, bool MAX_SQ, bool OPTI_ON, bool TURN_ON, int ALG)
	{
		Parts = CParts;
		SetNewSheetSize(order.parameters.ListLength_mm, order.parameters.ListWidth_mm, order.parameters.Blade, order.parameters.Padding, DoublePadding: true);
		PARAMS = new LW16(SAME_MAX, MAX_SQ, OPTI_ON, TURN_ON);
		alg = ALG;
		l16 = true;
		PSQ = order.PartsSq;
	}

	public GetSheet_ThreadOperator(List<CPart> CParts, Order order, int ALG)
	{
		Parts = CParts;
		SetNewSheetSize(order.parameters.ListLength_mm, order.parameters.ListWidth_mm, order.parameters.Blade, order.parameters.Padding, DoublePadding: true);
		alg = ALG;
		l16 = false;
	}

	public void SetNewSheetSize(float ListLength, float ListWidth, float Blade, float Padding, bool DoublePadding)
	{
		LL = (int)ListLength * 10;
		LW = (int)ListWidth * 10;
		B = (int)Blade * 10;
		P = (int)Padding * 10;
		DP = DoublePadding;
	}

	public void SetCutParams_L16(bool SAME_MAX, bool MAX_SQ, bool OPTI_ON, bool TURN_ON)
	{
		PARAMS.SAME_MAX = SAME_MAX;
		PARAMS.MAX_SQ = MAX_SQ;
		PARAMS.OPTI_ON = OPTI_ON;
		PARAMS.TURN_ON = TURN_ON;
	}

	public void GET_SHEET_THREAD()
	{
		switch (alg)
		{
		case 1:
		{
			if (l16)
			{
				Length2 length = new Length2();
				Sheet = length.GetCSheet_LENGTH_CUT(Parts, LL, LW, B, P, DP, PARAMS, PSQ, PPSQ, out PPSQ);
				break;
			}
			Length_Alg length_Alg = new Length_Alg();
			CSheet cSheet_LENGTH_CUT = length_Alg.GetCSheet_LENGTH_CUT(Parts, LL, LW, B, P, DP, Opti_ON: true, CleanParts: true);
			SET_OFF_Parts_in_Sheet(Parts, cSheet_LENGTH_CUT);
			CSheet cSheet_LENGTH_CUT2 = length_Alg.GetCSheet_LENGTH_CUT(Parts, LL, LW, B, P, DP, Opti_ON: false, CleanParts: false);
			if ((int)((cSheet_LENGTH_CUT2.Parts_Sq - cSheet_LENGTH_CUT.Parts_Sq) * 100.0) > 0 || ((int)((cSheet_LENGTH_CUT2.Parts_Sq - cSheet_LENGTH_CUT.Parts_Sq) * 100.0) == 0 && (cSheet_LENGTH_CUT2.Remain.W - cSheet_LENGTH_CUT.Remain.W) * 100 > 0))
			{
				Sheet = cSheet_LENGTH_CUT2;
				break;
			}
			SET_OFF_Parts_in_Sheet(Parts, cSheet_LENGTH_CUT2);
			SET_ON_Parts_in_Sheet(Parts, cSheet_LENGTH_CUT);
			Sheet = cSheet_LENGTH_CUT;
			break;
		}
		case 2:
		{
			if (l16)
			{
				Width2 width = new Width2();
				Sheet = width.GetCSheet_WIDTH_CUT(Parts, LL, LW, B, P, DP, PARAMS, PSQ, PPSQ, out PPSQ);
				break;
			}
			Width_Alg width_Alg = new Width_Alg();
			CSheet cSheet_WIDTH_CUT = width_Alg.GetCSheet_WIDTH_CUT(Parts, LL, LW, B, P, DP, Opti_ON: true, CleanParts: true);
			SET_OFF_Parts_in_Sheet(Parts, cSheet_WIDTH_CUT);
			CSheet cSheet_WIDTH_CUT2 = width_Alg.GetCSheet_WIDTH_CUT(Parts, LL, LW, B, P, DP, Opti_ON: false, CleanParts: false);
			if ((int)((cSheet_WIDTH_CUT2.Parts_Sq - cSheet_WIDTH_CUT.Parts_Sq) * 100.0) > 0 || ((int)((cSheet_WIDTH_CUT2.Parts_Sq - cSheet_WIDTH_CUT.Parts_Sq) * 100.0) == 0 && (cSheet_WIDTH_CUT2.Remain.L - cSheet_WIDTH_CUT.Remain.L) * 100 > 0))
			{
				Sheet = cSheet_WIDTH_CUT2;
				break;
			}
			SET_OFF_Parts_in_Sheet(Parts, cSheet_WIDTH_CUT2);
			SET_ON_Parts_in_Sheet(Parts, cSheet_WIDTH_CUT);
			Sheet = cSheet_WIDTH_CUT;
			break;
		}
		case 3:
		{
			Opt_Alg_Width_and_Length opt_Alg_Width_and_Length = new Opt_Alg_Width_and_Length();
			Sheet = opt_Alg_Width_and_Length.Get_Sheet_OPT_ALG_2(Parts, LL, LW, B, P, DP, SAME_MAX: true, MAX_SQ: false, OPTI_ON: true, TURN_ON: true, 3);
			break;
		}
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

	public bool CheckParts_for_ActualSheet()
	{
		bool result = false;
		int num = LL - P;
		int num2 = LW - P;
		if (DP)
		{
			num = LL - P;
			num2 = LW - P;
		}
		if (num > 0 && num2 > 0)
		{
			for (int num3 = Parts.Count - 1; num3 >= 0; num3--)
			{
				if (Parts[num3].Plased < Parts[num3].Qty && ((num >= Parts[num3].L && num2 >= Parts[num3].W) || (Parts[num3].Turn && num >= Parts[num3].W && num2 >= Parts[num3].L)))
				{
					result = true;
					num3 = -1;
				}
			}
		}
		return result;
	}
}
