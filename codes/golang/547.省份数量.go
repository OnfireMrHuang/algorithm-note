

type UnionFind struct {
	parent []int
	count  int
	size   []int
}

func NewUnionFind(n int) *UnionFind {
	uf := &UnionFind{}
	uf.parent = make([]int, n)
	uf.size = make([]int, n)
	uf.count = n
	for i := 0; i < n; i++ {
		uf.parent[i] = i
		uf.size[i] = 1
	}
	return uf
}

func (uf *UnionFind) Find(p int) int {
	for p != uf.parent[p] {
		uf.parent[p] = uf.parent[uf.parent[p]]
		p = uf.parent[p]
	}
	return p
}

func (uf *UnionFind) Union(p, q int) {
	rootP := uf.Find(p)
	rootQ := uf.Find(q)
	if rootP == rootQ {
		return
	}
	if uf.size[rootP] > uf.size[rootQ] {
		uf.parent[rootQ] = rootP
		uf.size[rootP] += uf.size[rootQ]
	} else {
		uf.parent[rootP] = rootQ
		uf.size[rootQ] += uf.size[rootP]
	}
	uf.count--
}

func (uf *UnionFind) Count() int {
	return uf.count
}

func findCircleNum(isConnected [][]int) int {
	uf := NewUnionFind(len(isConnected))
	for i := 0; i < len(isConnected); i++ {
		for j := i + 1; j < len(isConnected); j++ {
			if isConnected[i][j] == 1 {
				uf.Union(i, j)
			}
		}
	}
	return uf.Count()
}