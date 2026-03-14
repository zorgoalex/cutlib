using System;
using System.Collections.Generic;
using System.Globalization;


public class Utils
{
	private static int PartsCount;

	private static long MaxSnipSq = 1000000000L;

	private const string SPACE = " ";

	private const string INCH = "\"";

	private const string SLASH = "/";

	private const string DOT = ".";

	private static int gray = 13882323;

	private static int white = 16777215;

	private static int black = 0;

	private static int darkgray = 11119017;

	private static int limit = 780;

	private static int PageNumber = 0;

	private static Order R_O = new Order();

	private static int[,,,] NotTurn;

	private static int[,,,] IsTurn;

	public static string ConvertSize(string Size, int Units, int NewUnits)
	{
		string result = "";
		NumberFormatInfo invariantInfo = NumberFormatInfo.InvariantInfo;
		CultureInfo invariantCulture = CultureInfo.InvariantCulture;
		MidpointRounding mode = MidpointRounding.AwayFromZero;
		int digits = 2;
		int digits2 = 3;
		switch (Units)
		{
		case 1:
			switch (NewUnits)
			{
			case 1:
				result = Size;
				break;
			case 2:
				result = Math.Round((float)int.Parse(Size, invariantCulture.NumberFormat) / 25.4f, digits2, mode).ToString(invariantInfo) ?? "";
				break;
			case 3:
			{
				int num17 = int.Parse(Size);
				int num18 = 64;
				int num19 = (int)((float)num17 / 25.4f);
				int num20 = (int)(((float)num17 - (float)num19 * 25.4f) / 25.4f * (float)num18);
				if (num20 != 0)
				{
					bool flag2 = true;
					while (flag2)
					{
						if (num20 % 2 == 0 && num20 != 0)
						{
							num20 /= 2;
							num18 /= 2;
						}
						else
						{
							flag2 = false;
						}
					}
					result = num19 + " " + num20 + "/" + num18;
				}
				else
				{
					result = string.Concat(num19);
				}
				break;
			}
			case 4:
				result = Math.Round(double.Parse(Size, invariantCulture.NumberFormat) / 10.0, digits, mode).ToString(invariantInfo) ?? "";
				break;
			}
			break;
		case 4:
			switch (NewUnits)
			{
			case 1:
				result = string.Concat((int)Math.Round(double.Parse(Size, invariantCulture.NumberFormat) * 10.0));
				break;
			case 2:
				result = Math.Round(float.Parse(Size, invariantCulture.NumberFormat) / 2.54f, digits2, mode).ToString(invariantInfo) ?? "";
				break;
			case 3:
			{
				float num13 = float.Parse(Size, invariantCulture.NumberFormat);
				int num14 = 64;
				int num15 = (int)(num13 / 2.54f);
				int num16 = (int)((num13 - (float)num15 * 2.54f) / 2.54f * (float)num14);
				if (num16 != 0)
				{
					bool flag = true;
					while (flag)
					{
						if (num16 % 2 == 0 && num16 != 0)
						{
							num16 /= 2;
							num14 /= 2;
						}
						else
						{
							flag = false;
						}
					}
					result = num15 + " " + num16 + "/" + num14;
				}
				else
				{
					result = string.Concat(num15);
				}
				break;
			}
			case 4:
				result = Size;
				break;
			}
			break;
		case 2:
			if (Size.IndexOf("\"") != -1)
			{
				Size = Size.Remove(Size.IndexOf("\""), 1);
			}
			switch (NewUnits)
			{
			case 1:
				result = string.Concat((int)Math.Round(float.Parse(Size, invariantCulture.NumberFormat) * 25.4f, mode));
				break;
			case 2:
				result = Size;
				break;
			case 3:
			{
				float num21 = float.Parse(Size, invariantCulture.NumberFormat);
				int num22 = 64;
				int num23 = (int)Math.Truncate(num21);
				int num24 = (int)Math.Round((num21 - (float)num23) * (float)num22);
				if (num24 == 0)
				{
					result = Size;
					break;
				}
				bool flag3 = true;
				while (flag3)
				{
					if (num24 % 2 == 0 && num24 != 0)
					{
						num24 /= 2;
						num22 /= 2;
					}
					else
					{
						flag3 = false;
					}
				}
				result = num23 + " " + num24 + "/" + num22;
				break;
			}
			case 4:
				result = Math.Round(double.Parse(Size, invariantCulture.NumberFormat) * 2.5399999618530273, 2, mode).ToString(invariantInfo) ?? "";
				break;
			}
			break;
		case 3:
			if (Size.IndexOf("\"") != -1)
			{
				Size = Size.Remove(Size.IndexOf("\""), 1);
			}
			switch (NewUnits)
			{
			case 1:
			{
				float num5 = 0f;
				float num6 = 0f;
				if (Size.IndexOf(" ") != -1 && Size.IndexOf(" ") < Size.Length - 3)
				{
					num5 = (float)Math.Round(float.Parse(Size.Substring(0, Size.IndexOf(" ")), invariantCulture.NumberFormat) * 25.4f, 2, mode);
				}
				if (Size.IndexOf(" ") == -1 && Size.IndexOf("/") == -1)
				{
					num5 = (float)Math.Round(float.Parse(Size, NumberFormatInfo.InvariantInfo) * 25.4f, 2, mode);
				}
				if (Size.IndexOf(" ") != -1 && Size.IndexOf(" ") == Size.Length - 1)
				{
					Size = Size.Remove(Size.IndexOf(" "), 1);
					num5 = (float)Math.Round(float.Parse(Size, NumberFormatInfo.InvariantInfo) * 25.4f, 2, mode);
				}
				if (Size.IndexOf("/") != -1)
				{
					float num7 = float.Parse(Size.Substring(Size.IndexOf(" ") + 1, Size.IndexOf("/") - Size.IndexOf(" ") - 1));
					float num8 = float.Parse(Size.Substring(Size.IndexOf("/") + 1, Size.Length - Size.IndexOf("/") - 1));
					num6 = (float)Math.Round(num7 / num8 * 25.4f, 2, mode);
				}
				num6 += num5;
				result = string.Concat((int)Math.Round(num6, mode));
				break;
			}
			case 4:
			{
				float num9 = 0f;
				float num10 = 0f;
				if (Size.IndexOf(" ") != -1 && Size.IndexOf(" ") < Size.Length - 3)
				{
					num9 = (float)Math.Round(float.Parse(Size.Substring(0, Size.IndexOf(" ")), invariantCulture.NumberFormat) * 2.54f, 2, mode);
				}
				if (Size.IndexOf(" ") == -1 && Size.IndexOf("/") == -1)
				{
					num9 = (float)Math.Round(float.Parse(Size, NumberFormatInfo.InvariantInfo) * 2.54f, 2, mode);
				}
				if (Size.IndexOf(" ") != -1 && Size.IndexOf(" ") == Size.Length - 1)
				{
					Size = Size.Remove(Size.IndexOf(" "), 1);
					num9 = (float)Math.Round(float.Parse(Size, NumberFormatInfo.InvariantInfo) * 2.54f, 2, mode);
				}
				if (Size.IndexOf("/") != -1)
				{
					float num11 = float.Parse(Size.Substring(Size.IndexOf(" ") + 1, Size.IndexOf("/") - Size.IndexOf(" ") - 1));
					float num12 = float.Parse(Size.Substring(Size.IndexOf("/") + 1, Size.Length - Size.IndexOf("/") - 1));
					num10 = (float)Math.Round(num11 / num12 * 2.54f, 2, mode);
				}
				result = ((float)Math.Round(num10 + num9, 1, mode)).ToString(invariantInfo) ?? "";
				break;
			}
			case 2:
			{
				if (Size.IndexOf("\"") != -1)
				{
					Size = Size.Remove(Size.IndexOf("\""));
				}
				float num = 0f;
				float num2 = 0f;
				if (Size.IndexOf(" ") != -1)
				{
					num = (int)Math.Round(float.Parse(Size.Substring(0, Size.IndexOf(" ")), NumberFormatInfo.InvariantInfo));
				}
				else if (Size.IndexOf("/") == -1)
				{
					num = (int)Math.Round(float.Parse(Size, NumberFormatInfo.InvariantInfo));
				}
				if (Size.IndexOf("/") != -1)
				{
					int num3 = int.Parse(Size.Substring(Size.IndexOf(" ") + 1, Size.IndexOf("/") - Size.IndexOf(" ") - 1));
					int num4 = int.Parse(Size.Substring(Size.IndexOf("/") + 1, Size.Length - Size.IndexOf("/") - 1));
					num2 = (float)Math.Round((float)num3 / (float)num4, 3, MidpointRounding.AwayFromZero);
				}
				result = (((int)(num2 * 1000f) <= 0) ? string.Concat(num) : ((num + num2).ToString(NumberFormatInfo.InvariantInfo) ?? ""));
				break;
			}
			case 3:
				result = Size;
				break;
			}
			break;
		}
		return result;
	}

	public static string ConverMMtoFeets(int mm)
	{
		return Math.Round((float)mm / 304.8f, 2).ToString(NumberFormatInfo.InvariantInfo);
	}

	public static string GetSq_in_KV_M(string Sq)
	{
		return Math.Round(float.Parse(Sq) / 1000000f, 3).ToString(NumberFormatInfo.InvariantInfo);
	}

	public static string GetSq_in_SQ_Feets(string Sq)
	{
		return Math.Round(float.Parse(Sq) / 1000000f * 10.7639f, 3).ToString(NumberFormatInfo.InvariantInfo) ?? "";
	}

	public static string Correct_String(string Str, int Units)
	{
		if (Units != 3)
		{
			bool flag = false;
			for (int i = 0; i < Str.Length; i++)
			{
				if (!flag)
				{
					if (!char.IsNumber(Str[i]) && !Str[i].Equals('.') && !Str[i].Equals(','))
					{
						Str = Str.Remove(i, 1);
						i--;
					}
					else if (Str[i].Equals('.') || Str[i].Equals(','))
					{
						flag = true;
					}
				}
				else if (!char.IsNumber(Str[i]))
				{
					Str = Str.Remove(i, 1);
					i--;
				}
			}
			int num = Str.IndexOf(',');
			if (num != -1)
			{
				Str = Str.Substring(0, num) + "." + Str.Substring(num + 1);
			}
		}
		return Str;
	}

	public static int CheckString(string Size, int Units, float MaxLimit)
	{
		int result = 0;
		Size = Correct_String(Size, Units);
		switch (Units)
		{
		case 1:
			if (Size.Length == 0)
			{
				result = 4;
				break;
			}
			try
			{
				float num2 = float.Parse(Size, CultureInfo.InvariantCulture);
				result = (((int)(num2 * 100f) == 0) ? 3 : (((int)((MaxLimit - num2) * 100f) < 0) ? 1 : 0));
			}
			catch
			{
				result = 2;
			}
			break;
		case 4:
			if (Size.Length == 0)
			{
				result = 4;
				break;
			}
			if (Size.Equals("0 ") || Size.Equals("0") || Size.Equals("0."))
			{
				result = 3;
				break;
			}
			try
			{
				float num4 = float.Parse(ConvertSize(Size, Units, 1));
				result = (((int)(num4 * 100f) == 0) ? 3 : (((int)((MaxLimit - num4) * 100f) < 0) ? 1 : 0));
			}
			catch
			{
				result = 2;
			}
			break;
		case 2:
			if (Size.Length > 0 && Size.IndexOf("\"") != -1)
			{
				Size = Size.Remove(Size.IndexOf("\""), 1);
			}
			if (Size.Length == 0)
			{
				result = 4;
				break;
			}
			if (Size.Equals("0 ") || Size.Equals("0") || Size.Equals("0."))
			{
				result = 3;
				break;
			}
			try
			{
				float num3 = float.Parse(ConvertSize(Size, Units, 1));
				result = (((int)(num3 * 100f) == 0) ? 3 : (((int)((MaxLimit - num3) * 100f) < 0) ? 1 : 0));
			}
			catch
			{
				result = 2;
			}
			break;
		case 3:
			if (Size.Length > 0 && Size.IndexOf("\"") != -1)
			{
				Size = Size.Remove(Size.IndexOf("\""), 1);
			}
			if (Size.Length == 0)
			{
				result = 4;
			}
			else if (Size.Equals("0 ") || Size.Equals("0.") || Size.Equals("0"))
			{
				result = 3;
			}
			else if ((Size.IndexOf(" ") != -1 && Size.IndexOf("/") == -1 && Size.IndexOf(" ") == Size.Length - 1) || (Size.IndexOf(" ") == -1 && Size.IndexOf("/") == -1 && Size.Length <= 3) || (Size.IndexOf("/") != -1 && Size.IndexOf("/") != Size.Length - 1) || Size.IndexOf("/") >= Size.Length - 3)
			{
				if (Size.Length > 0 && Size.IndexOf("\"") == -1)
				{
					Size += "\"";
				}
				try
				{
					float num = float.Parse(ConvertSize(Size, Units, 1));
					result = (((int)(num * 100f) == 0) ? 3 : (((int)((MaxLimit - num) * 100f) < 0) ? 1 : 0));
				}
				catch
				{
					result = 2;
				}
			}
			else
			{
				result = 2;
			}
			break;
		}
		return result;
	}

	public static Order ClearCuttingInfo(Order order)
	{
		order.PartsPlased = 0;
		order.SheetCount = 0;
		order.UsedSnipsCount = 0;
		for (int i = 0; i < order.Parts.Count; i++)
		{
			order.Parts[i].nPlased = 0;
			for (int j = 0; j < order.Parts[i].Amount; j++)
			{
				order.Parts[i].Coords[j].Cutted = false;
				order.Parts[i].Coords[j].isTurn = false;
				order.Parts[i].Coords[j].list = -1;
				order.Parts[i].Coords[j].nlist = -1;
				order.Parts[i].Coords[j].onList = false;
				order.Parts[i].Coords[j].X = -1;
				order.Parts[i].Coords[j].Y = -1;
			}
		}
		for (int k = 0; k < order.Snips.Count; k++)
		{
			order.Snips[k].nCutted = 0;
			order.Snips[k].nDrawed = 0;
		}
		order.NSnips.Clear();
		order.PartsCount = GetPartsCount(order);
		order.PartsSq = GetPartsSq(order);
		return order;
	}

	public static int GetNumberIndex(string str)
	{
		int result = -1;
		for (int i = 0; i < str.Length; i++)
		{
			if (str[i] >= '0' && str[i] <= '9')
			{
				result = i;
				i = str.Length;
			}
			else
			{
				result = -1;
			}
		}
		return result;
	}

	public static int[] GetMaxLengthAndWidth(Order o)
	{
		int[] result = new int[] { o.parameters.ListLength_mm, o.parameters.ListWidth_mm };
		for (int i = 0; i < o.Snips.Count; i++)
		{
			if (o.Snips[i].Length_mm > result[0])
			{
				result[0] = o.Snips[i].Length_mm;
			}
			if (o.Snips[i].Width_mm > result[1])
			{
				result[1] = o.Snips[i].Width_mm;
			}
		}
		return result;
	}

	public static void ResizeSnip(Order o, int io, int X, int Y, int length, int width)
	{
		if (length > 0 && width > 0)
		{
			o.NSnips[io].Length_mm = length;
			o.NSnips[io].Width_mm = width;
			o.NSnips[io].X = X;
			o.NSnips[io].Y = Y;
			o.NSnips[io].onList = true;
			o.NSnips[io].Sq = length * width;
		}
		else
		{
			o.NSnips[io].Length_mm = length;
			o.NSnips[io].Width_mm = width;
			o.NSnips[io].X = X;
			o.NSnips[io].Y = Y;
			o.NSnips[io].onList = true;
			o.NSnips[io].full = true;
			o.NSnips[io].Sq = 0L;
		}
	}

	public static int GetPartsCount(Order o)
	{
		int num = 0;
		for (int i = 0; i < o.Parts.Count; i++)
		{
			num += o.Parts[i].Amount;
		}
		return num;
	}

	public static int GetUsedSnipsCount(Order o)
	{
		int num = 0;
		for (int i = 0; i < o.Snips.Count; i++)
		{
			num += o.Snips[i].nCutted;
		}
		return num;
	}

	public static long GetPartsSq(Order o)
	{
		long num = 0L;
		for (int i = 0; i < o.Parts.Count; i++)
		{
			num += o.Parts[i].Amount * o.Parts[i].Sq;
		}
		return num;
	}

	public static int GetSnipsCount(Order o)
	{
		int num = 0;
		for (int i = 0; i < o.Snips.Count; i++)
		{
			num += o.Snips[i].Amount;
		}
		return num;
	}

	public static int GetPartsPlased(Order o)
	{
		int num = 0;
		for (int i = 0; i < o.Parts.Count; i++)
		{
			num += o.Parts[i].nPlased;
		}
		return num;
	}

	public static int GetSheetsCount(Order o)
	{
		int num = 0;
		for (int i = 0; i < o.Parts.Count; i++)
		{
			for (int j = 0; j < o.Parts[i].nPlased; j++)
			{
				if (num < o.Parts[i].Coords[j].list)
				{
					num = o.Parts[i].Coords[j].list;
				}
			}
		}
		return num;
	}

	public static long GetMaxSquareOfSnips(Order o)
	{
		long num = o.parameters.ListLength_mm * o.parameters.ListWidth_mm;
		for (int i = 0; i < o.Snips.Count; i++)
		{
			if (num < o.Snips[i].Length_mm * o.Snips[i].Width_mm)
			{
				num = o.Snips[i].Length_mm * o.Snips[i].Width_mm;
			}
		}
		return num;
	}

	public static long GetNSnipsSq(Order o)
	{
		long num = 0L;
		for (int i = 0; i < o.NSnips.Count; i++)
		{
			if (o.NSnips[i].Sq > 20000)
			{
				num += o.NSnips[i].Sq;
			}
		}
		return num;
	}

	public static int CreateNew_NSnip(Order o, int list, int nlist, int X, int Y, int length, int width)
	{
		Snip snip = new Snip();
		snip.Length_mm = length;
		snip.Width_mm = width;
		snip.Sq = length * width;
		snip.X = X;
		snip.Y = Y;
		snip.onList = true;
		snip.list = list;
		snip.nlist = nlist;
		if (length <= 0 || width <= 0)
		{
			if (snip.Sq > 0)
			{
				snip.Sq = length * width * -1;
			}
			snip.full = true;
		}
		o.NSnips.Add(snip);
		return o.NSnips.Count - 1;
	}

	private static int GetCutLength(Order o, int i, int co)
	{
		int result = 0;
		int num = o.Parts[i].Length_mm;
		int num2 = o.Parts[i].Width_mm;
		int listLength_mm = o.parameters.ListLength_mm;
		int listWidth_mm = o.parameters.ListWidth_mm;
		int padding = o.parameters.Padding;
		if (o.Parts[i].Coords[co].isTurn)
		{
			num = o.Parts[i].Width_mm;
			num2 = o.Parts[i].Length_mm;
		}
		if (o.Parts[i].Coords[co].list > 0)
		{
			if (padding == 0)
			{
				if (num == listLength_mm && num2 != listWidth_mm)
				{
					result = num;
				}
				else if (num2 == listWidth_mm && num != listLength_mm)
				{
					result = num2;
				}
				else if (num == listLength_mm && num2 == listWidth_mm)
				{
					result = 0;
				}
				else if (num != listLength_mm && num2 != listWidth_mm)
				{
					result = num + num2;
				}
			}
			else
			{
				result = num + num2;
			}
		}
		else if (o.Parts[i].Coords[co].list < -1)
		{
			int index = o.Parts[i].Coords[co].list * -1 - 2;
			listLength_mm = o.Snips[index].Length_mm;
			listWidth_mm = o.Snips[index].Width_mm;
			if (padding == 0)
			{
				if (num == listLength_mm && num2 != listWidth_mm)
				{
					result = num;
				}
				else if (num2 == listWidth_mm && num != listLength_mm)
				{
					result = num2;
				}
				else if (num == listLength_mm && num2 == listWidth_mm)
				{
					result = 0;
				}
				else if (num != listLength_mm && num2 != listWidth_mm)
				{
					result = num + num2;
				}
			}
			else
			{
				result = num + num2;
			}
		}
		else
		{
			result = 0;
		}
		return result;
	}

	private static int GetSnipCutLength(Order o, int i)
	{
		int result = 0;
		int length_mm = o.NSnips[i].Length_mm;
		int width_mm = o.NSnips[i].Width_mm;
		int listLength_mm = o.parameters.ListLength_mm;
		int listWidth_mm = o.parameters.ListWidth_mm;
		int padding = o.parameters.Padding;
		int x = o.NSnips[i].X;
		int y = o.NSnips[i].Y;
		if (o.NSnips[i].list > 0)
		{
			if (padding == 0)
			{
				if (length_mm == listLength_mm || width_mm == listWidth_mm)
				{
					result = 0;
				}
				else if (x + length_mm == listLength_mm && y + width_mm == listWidth_mm)
				{
					result = 0;
				}
				else if (x + length_mm == listLength_mm && y + width_mm != listWidth_mm)
				{
					result = length_mm;
				}
				else if (x + length_mm != listLength_mm && y + width_mm == listWidth_mm)
				{
					result = width_mm;
				}
				else if (x + length_mm != listLength_mm && y + width_mm != listWidth_mm)
				{
					result = length_mm + width_mm;
				}
			}
			else
			{
				result = length_mm + width_mm;
			}
		}
		else if (o.NSnips[i].list < -1)
		{
			int index = o.NSnips[i].list * -1 - 2;
			listLength_mm = o.Snips[index].Length_mm;
			listWidth_mm = o.Snips[index].Width_mm;
			if (padding == 0 || !o.Snips[index].offcut)
			{
				if (length_mm == listLength_mm || width_mm == listWidth_mm)
				{
					result = 0;
				}
				else if (x + length_mm == listLength_mm && y + width_mm == listWidth_mm)
				{
					result = 0;
				}
				else if (x + length_mm == listLength_mm && y + width_mm != listWidth_mm)
				{
					result = length_mm;
				}
				else if (x + length_mm != listLength_mm && y + width_mm == listWidth_mm)
				{
					result = width_mm;
				}
				else if (x + length_mm != listLength_mm && y + width_mm != listWidth_mm)
				{
					result = length_mm + width_mm;
				}
			}
			else
			{
				result = length_mm + width_mm;
			}
		}
		else
		{
			result = 0;
		}
		return result;
	}

	public static Order GetResultsOrder(Order o)
	{
		o.SheetSq = o.parameters.ListLength_mm * o.parameters.ListWidth_mm;
		o.Sheets_Sq = o.SheetSq * o.SheetCount;
		o.UsedSnipsSq = 0L;
		o.L_Cuts = 0;
		o.PartsCount = 0;
		o.PartsPlased = 0;
		o.PartsSq = 0L;
		o.NSnips_Sq = 0L;
		o.Waste_Sq = 0L;
		o.L_Edging1 = 0;
		o.L_Edging2 = 0;
		o.L_Slots = 0;
		for (int i = 0; i < o.Snips.Count; i++)
		{
			int nCutted = o.Snips[i].nCutted;
			o.UsedSnipsSq += nCutted * o.Snips[i].Length_mm * o.Snips[i].Width_mm;
		}
		for (int j = 0; j < o.Parts.Count; j++)
		{
			for (int k = 0; k < o.Parts[j].Amount; k++)
			{
				o.L_Cuts += GetCutLength(o, j, k);
			}
		}
		for (int l = 0; l < o.NSnips.Count; l++)
		{
			o.L_Cuts += GetSnipCutLength(o, l);
		}
		if (o.parameters.Padding != 0)
		{
			o.L_Cuts += (o.parameters.ListLength_mm + o.parameters.ListWidth_mm) * o.SheetCount;
			for (int m = 0; m < o.Snips.Count; m++)
			{
				if (o.Snips[m].nCutted > 0 && o.Snips[m].offcut)
				{
					o.L_Cuts += (o.Snips[m].Length_mm + o.Snips[m].Width_mm) * o.Snips[m].nCutted;
				}
			}
		}
		for (int n = 0; n < o.Parts.Count; n++)
		{
			o.PartsSq += o.Parts[n].Length_mm * o.Parts[n].Width_mm * o.Parts[n].Amount;
			o.PartsCount += o.Parts[n].Amount;
			o.PartsPlased += o.Parts[n].nPlased;
			switch (o.Parts[n].ELength)
			{
			case 1:
				o.L_Edging1 += o.Parts[n].Length_mm * o.Parts[n].Amount;
				break;
			case 2:
				o.L_Edging1 += 2 * o.Parts[n].Length_mm * o.Parts[n].Amount;
				break;
			case 3:
				o.L_Edging2 += o.Parts[n].Length_mm * o.Parts[n].Amount;
				break;
			case 4:
				o.L_Edging2 += 2 * o.Parts[n].Length_mm * o.Parts[n].Amount;
				break;
			case 5:
				o.L_Edging1 += o.Parts[n].Length_mm * o.Parts[n].Amount;
				o.L_Edging2 += o.Parts[n].Length_mm * o.Parts[n].Amount;
				break;
			}
			switch (o.Parts[n].EWidth)
			{
			case 1:
				o.L_Edging1 += o.Parts[n].Width_mm * o.Parts[n].Amount;
				break;
			case 2:
				o.L_Edging1 += 2 * o.Parts[n].Width_mm * o.Parts[n].Amount;
				break;
			case 3:
				o.L_Edging2 += o.Parts[n].Width_mm * o.Parts[n].Amount;
				break;
			case 4:
				o.L_Edging2 += 2 * o.Parts[n].Width_mm * o.Parts[n].Amount;
				break;
			case 5:
				o.L_Edging1 += o.Parts[n].Width_mm * o.Parts[n].Amount;
				o.L_Edging2 += o.Parts[n].Width_mm * o.Parts[n].Amount;
				break;
			}
			o.L_Slots += o.Parts[n].Length_mm * o.Parts[n].Amount * o.Parts[n].ESlotsLength;
			o.L_Slots += o.Parts[n].Width_mm * o.Parts[n].Amount * o.Parts[n].ESlotsWidth;
		}
		for (int num = 0; num < o.NSnips.Count; num++)
		{
			long num2 = o.NSnips[num].Length_mm * o.NSnips[num].Width_mm;
			if (num2 > 50000)
			{
				o.NSnips_Sq += num2;
			}
		}
		if (o.PartsPlased > 0)
		{
			o.Waste_Sq = o.Sheets_Sq + o.UsedSnipsSq - o.PartsSq - o.NSnips_Sq;
		}
		else
		{
			o.Waste_Sq = 0L;
		}
		return o;
	}

	private static string GetSqInUnits(long Sq, int Units)
	{
		float num = (float)Math.Round((float)Sq / 1000000f, 3);
		if (Units == 1)
		{
			return num.ToString();
		}
		return ((float)Math.Round(num * 10.763f, 3)).ToString();
	}

	private static string GetLengthInUnits(int L, int Units)
	{
		float num = (float)Math.Round((float)L / 1000f, 2);
		if (Units == 1)
		{
			return num.ToString();
		}
		return ((float)Math.Round(num * 3.28f, 2)).ToString();
	}

	public static void GetCheckedSheets(Order o, bool HideTheSame, out List<int> CheckedQty, out List<List<ChPart>> CheckedParts, out List<List<ChPart>> CheckedSnips)
	{
		CheckedParts = new List<List<ChPart>>();
		CheckedSnips = new List<List<ChPart>>();
		CheckedQty = new List<int>();
		if (o.SheetCount <= 0)
		{
			return;
		}
		for (int i = 0; i < o.SheetCount; i++)
		{
			CheckedParts.Add(new List<ChPart>());
			CheckedSnips.Add(new List<ChPart>());
		}
		for (int j = 0; j < o.Parts.Count; j++)
		{
			for (int k = 0; k < o.Parts[j].Amount; k++)
			{
				if (o.Parts[j].Coords[k].onList && o.Parts[j].Coords[k].list > 0)
				{
					ChPart chPart = new ChPart();
					chPart.iD = j;
					chPart.L = o.Parts[j].Length_mm;
					chPart.W = o.Parts[j].Width_mm;
					chPart.co = k;
					chPart.X = o.Parts[j].Coords[k].X;
					chPart.Y = o.Parts[j].Coords[k].Y;
					chPart.list = o.Parts[j].Coords[k].list;
					chPart.nlist = o.Parts[j].Coords[k].nlist;
					chPart.isTurn = o.Parts[j].Coords[k].isTurn;
					CheckedParts[chPart.list - 1].Add(chPart);
				}
			}
		}
		for (int l = 0; l < o.NSnips.Count; l++)
		{
			if (o.NSnips[l].onList && o.NSnips[l].list >= 0)
			{
				ChPart chPart2 = new ChPart();
				chPart2.iD = l;
				chPart2.L = o.NSnips[l].Length_mm;
				chPart2.W = o.NSnips[l].Width_mm;
				chPart2.X = o.NSnips[l].X;
				chPart2.Y = o.NSnips[l].Y;
				chPart2.list = o.NSnips[l].list;
				chPart2.nlist = o.NSnips[l].nlist;
				CheckedSnips[chPart2.list - 1].Add(chPart2);
			}
		}
		for (int m = 0; m < CheckedParts.Count; m++)
		{
			int num = 1;
			if (HideTheSame)
			{
				bool flag = false;
				for (int n = m + 1; n < CheckedParts.Count; n++)
				{
					if (CheckedParts[m].Count != CheckedParts[n].Count)
					{
						continue;
					}
					List<ChPart> list = CheckedParts[m];
					List<ChPart> list2 = CheckedParts[n];
					flag = true;
					for (int num2 = 0; num2 < list.Count; num2++)
					{
						if (list[num2].iD != list2[num2].iD || list[num2].isTurn != list2[num2].isTurn || (int)((list[num2].X - list2[num2].X) * 10f) != 0 || (int)((list[num2].Y - list2[num2].Y) * 10f) != 0)
						{
							flag = false;
							num2 = list.Count;
						}
					}
					if (flag)
					{
						List<ChPart> list3 = CheckedSnips[m];
						List<ChPart> list4 = CheckedSnips[n];
						for (int num3 = 0; num3 < list3.Count; num3++)
						{
							if ((int)((list3[num3].X - list4[num3].X) * 10f) != 0 || (int)((list3[num3].Y - list4[num3].Y) * 10f) != 0 || (int)((list3[num3].L - list4[num3].L) * 10f) != 0 || (int)((list3[num3].W - list4[num3].W) * 10f) != 0)
							{
								flag = false;
								num3 = list3.Count;
							}
						}
					}
					if (flag)
					{
						num++;
						CheckedParts.Remove(CheckedParts[n]);
						CheckedSnips.Remove(CheckedSnips[n]);
						n--;
					}
				}
			}
			CheckedQty.Add(num);
		}
	}

	public static void GetCheckedSnips(Order o, bool HideTheSame, out List<int> CheckedID, out List<List<int>> CheckedQty, out List<List<List<ChPart>>> CheckedParts, out List<List<List<ChPart>>> CheckedSnips)
	{
		CheckedParts = new List<List<List<ChPart>>>();
		CheckedSnips = new List<List<List<ChPart>>>();
		CheckedQty = new List<List<int>>();
		CheckedID = new List<int>();
		for (int i = 0; i < o.Snips.Count; i++)
		{
			CheckedParts.Add(new List<List<ChPart>>());
			CheckedSnips.Add(new List<List<ChPart>>());
			CheckedQty.Add(new List<int>());
			CheckedID.Add(i);
			for (int j = 0; j < o.Snips[i].Amount; j++)
			{
				CheckedParts[i].Add(new List<ChPart>());
				CheckedSnips[i].Add(new List<ChPart>());
			}
		}
		for (int k = 0; k < o.Parts.Count; k++)
		{
			for (int l = 0; l < o.Parts[k].Amount; l++)
			{
				if (o.Parts[k].Coords[l].onList && o.Parts[k].Coords[l].list < -1)
				{
					ChPart chPart = new ChPart();
					chPart.iD = k;
					chPart.L = o.Parts[k].Length_mm;
					chPart.W = o.Parts[k].Width_mm;
					chPart.co = l;
					chPart.X = o.Parts[k].Coords[l].X;
					chPart.Y = o.Parts[k].Coords[l].Y;
					chPart.list = o.Parts[k].Coords[l].list;
					chPart.nlist = o.Parts[k].Coords[l].nlist;
					chPart.isTurn = o.Parts[k].Coords[l].isTurn;
					CheckedParts[chPart.list * -1 - 2][chPart.nlist - 1].Add(chPart);
				}
			}
		}
		for (int m = 0; m < o.NSnips.Count; m++)
		{
			if (o.NSnips[m].onList && o.NSnips[m].list < -1)
			{
				ChPart chPart2 = new ChPart();
				chPart2.iD = m;
				chPart2.L = o.NSnips[m].Length_mm;
				chPart2.W = o.NSnips[m].Width_mm;
				chPart2.X = o.NSnips[m].X;
				chPart2.Y = o.NSnips[m].Y;
				chPart2.list = o.NSnips[m].list;
				chPart2.nlist = o.NSnips[m].nlist;
				CheckedSnips[chPart2.list * -1 - 2][chPart2.nlist - 1].Add(chPart2);
			}
		}
		for (int n = 0; n < CheckedParts.Count; n++)
		{
			for (int num = 0; num < CheckedParts[n].Count; num++)
			{
				int num2 = 1;
				if (CheckedParts[n][num].Count == 0 && CheckedSnips[n][num].Count == 0)
				{
					num2 = 0;
				}
				else if (HideTheSame)
				{
					bool flag = false;
					for (int num3 = num + 1; num3 < CheckedParts[n].Count; num3++)
					{
						if (CheckedParts[n][num].Count != CheckedParts[n][num3].Count)
						{
							continue;
						}
						List<ChPart> list = CheckedParts[n][num];
						List<ChPart> list2 = CheckedParts[n][num3];
						flag = true;
						for (int num4 = 0; num4 < list.Count; num4++)
						{
							if (list[num4].iD != list2[num4].iD || list[num4].isTurn != list2[num4].isTurn || (int)((list[num4].X - list2[num4].X) * 10f) != 0 || (int)((list[num4].Y - list2[num4].Y) * 10f) != 0)
							{
								flag = false;
								num4 = list.Count;
							}
						}
						if (flag)
						{
							List<ChPart> list3 = CheckedSnips[n][num];
							List<ChPart> list4 = CheckedSnips[n][num3];
							if (list3.Count != list4.Count)
							{
								flag = false;
							}
							else
							{
								for (int num5 = 0; num5 < list3.Count; num5++)
								{
									if ((int)((list3[num5].X - list4[num5].X) * 10f) != 0 || (int)((list3[num5].Y - list4[num5].Y) * 10f) != 0 || (int)((list3[num5].L - list4[num5].L) * 10f) != 0 || (int)((list3[num5].W - list4[num5].W) * 10f) != 0)
									{
										flag = false;
										num5 = list3.Count;
									}
								}
							}
						}
						if (flag)
						{
							num2++;
							CheckedParts[n].Remove(CheckedParts[n][num3]);
							CheckedSnips[n].Remove(CheckedSnips[n][num3]);
							num3--;
						}
					}
				}
				CheckedQty[n].Add(num2);
			}
		}
	}

	public static float Get_PV_ScK(float LL, float LW)
	{
		float num = 100f / LL;
		float num2 = 75f / LW;
		if ((int)(num * 100f - num2 * 100f) < 0)
		{
			return num;
		}
		return num2;
	}
}
