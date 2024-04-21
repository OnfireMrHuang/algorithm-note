
type UnionFind struct {
	parent map[int]int
}

func NewUnionFind() *UnionFind {
	return &UnionFind{parent: make(map[int]int)}
}

func (uf *UnionFind) find(i int) int {
	if _, ok := uf.parent[i]; !ok {
		uf.parent[i] = i
		return i
	}
	if uf.parent[i] != i {
		uf.parent[i] = uf.find(uf.parent[i])
	}
	return uf.parent[i]
}

func (uf *UnionFind) union(p, q int) {
	parent := uf.find(p)
	children := uf.find(q)
	if parent == children {
		return
	}
	uf.parent[children] = parent
}

func findRedundantConnection(edges [][]int) []int {
	uf := NewUnionFind()
	for _, edg := range edges {
		parent := edg[0]
		children := edg[1]
		if uf.find(parent) == uf.find(children) {
			return edg
		}
		uf.union(parent, children)
	}
	return []int{}
}
