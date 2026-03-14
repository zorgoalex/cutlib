using System.Collections.Generic;

public class Order
{
	public Sheet sheet = new Sheet();
	public List<Part> Parts = new List<Part>();
	public List<Snip> Snips = new List<Snip>();
	public List<Snip> NSnips = new List<Snip>();
	public Parameters parameters = new Parameters();
	public int SheetCount;
	public int SnipsCount;
	public string _Order = "";
	public string Date = "";
	public string ReadyDate = "";
	public string Material = "";
	public string Thickness = "";
	public string Info = "";
	public int PartsPlased;
	public bool ChangeParameters;
	public bool ChangeParts;
	public bool ChangeSnips;
	public bool ChangeEdging;
	public bool ChangeMaps;
	public int MinLength;
	public int MinWidth;
	public string ActiveFile = "";
	public int UsedSnipsCount;
	public long UsedSnipsSq;
	public long SheetSq;
	public long Sheets_Sq;
	public int L_Cuts;
	public int PartsCount;
	public long PartsSq;
	public long NSnips_Sq;
	public long Waste_Sq;
	public int L_Edging1;
	public int L_Edging2;
	public int L_Slots;
}
