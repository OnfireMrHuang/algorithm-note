

type UF struct {
	count  int
	parent []int
	size   []int
}

func NewUF(n int) *UF {
	uf := &UF{
		count:  n,
		parent: make([]int, n),
		size:   make([]int, n),
	}
	for i := 0; i < n; i++ {
		uf.parent[i] = i
	}
	for i := 0; i < n; i++ {
		uf.size[i] = 1
	}
	return uf
}

func (uf *UF) Find(x int) int {
	for x != uf.parent[x] {
		uf.parent[x] = uf.parent[uf.parent[x]]
		x = uf.parent[x]
	}
	return x
}

func (uf *UF) Connected(x, y int) bool {
	xRoot := uf.Find(x)
	yRoot := uf.Find(y)
	return xRoot == yRoot
}

func (uf *UF) Union(x, y int) {
	xRoot := uf.Find(x)
	yRoot := uf.Find(y)
	if xRoot == yRoot {
		return
	}
	if uf.size[xRoot] > uf.size[yRoot] {
		uf.parent[yRoot] = xRoot
		uf.size[xRoot] += uf.size[yRoot]
	} else {
		uf.parent[xRoot] = yRoot
		uf.size[yRoot] += uf.size[xRoot]
	}
}

func makeConnected(n int, connections [][]int) int {
	uf := NewUF(n)
	idleConnection := 0
	unconnected := n - 1
	for _, connected := range connections {
		if uf.Connected(connected[0], connected[1]) {
			idleConnection++
		} else {
			uf.Union(connected[0], connected[1])
			unconnected--
		}
	}
	if idleConnection < unconnected {
		return -1
	}
	return unconnected
}
