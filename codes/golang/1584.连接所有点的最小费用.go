func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func minCostConnectPoints(points [][]int) int {
	n := len(points)
	edges := make([][]int, 0)
	for i := 0; i < n; i++ {
		for j := i + 1; j < n; j++ {
			xi, yi := points[i][0], points[i][1]
			xj, yj := points[j][0], points[j][1]
			edges = append(edges, []int{i, j, abs(xi-xj) + abs(yi-yj)})
		}
	}
	sort.Slice(edges, func(i, j int) bool {
		return edges[i][2] < edges[j][2]
	})
	mst := 0
	uf := NewUF(n)
	for _, edge := range edges {
		u, v, weight := edge[0], edge[1], edge[2]
		if uf.connected(u, v) {
			continue
		}
		mst += weight
		uf.union(u, v)
	}
	return mst
}

type UF struct {
	count  int
	parent []int
	size   []int
}

func NewUF(n int) *UF {
	uf := &UF{count: n}
	uf.parent = make([]int, n)
	uf.size = make([]int, n)
	for i := 0; i < n; i++ {
		uf.parent[i] = i
		uf.size[i] = 1
	}
	return uf
}

func (uf *UF) union(p, q int) {
	rootP := uf.find(p)
	rootQ := uf.find(q)
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

func (uf *UF) connected(p, q int) bool {
	return uf.find(p) == uf.find(q)
}

func (uf *UF) find(x int) int {
	for uf.parent[x] != x {
		uf.parent[x] = uf.parent[uf.parent[x]]
		x = uf.parent[x]
	}
	return x
}

func (uf *UF) Count() int {
	return uf.count
}


