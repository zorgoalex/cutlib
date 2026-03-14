using System.Collections.Generic;

public class CSheet
{
	public int L;

	public int W;

	public List<CLine> Lines = new List<CLine>();

	public List<int> Lines_index = new List<int>();

	public CSnip Remain;

	public Crd Remain_Crd;

	public double Parts_Sq;

	public List<CPart> parts;

	public bool Filled;

	public int list = -1;

	public int nlist = -1;

	public int Alg;

	public bool isFull;

	public double Sq => L * W;

	public float Filling => (float)(Parts_Sq / Sq);
}
