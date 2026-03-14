using System.Collections.Generic;

public class CLine
{
	public int L;

	public int W;

	public List<int> PartIDs;

	public List<Crd> Parts_Crds;

	public List<CSnip> Snips;

	public int SheetID = -1;

	public double Parts_Sq;

	public bool onSheet;

	public Crd crd;

	public double Sq => L * W;

	public float Filling => (float)(Parts_Sq / Sq);
}
