

type UnionFind struct {
	parent []int
	rank   []int
}

func NewUnionFind(n int) *UnionFind {
	parent := make([]int, n)
	rank := make([]int, n)
	for i := 0; i < n; i++ {
		parent[i] = i
		rank[i] = 1
	}
	return &UnionFind{parent, rank}
}

func (uf *UnionFind) find(x int) int {
	if x != uf.parent[x] {
		uf.parent[x] = uf.find(uf.parent[x])
	}
	return uf.parent[x]
}

func (uf *UnionFind) union(x, y int) bool {
	rootX := uf.find(x)
	rootY := uf.find(y)
	if rootX == rootY {
		return false
	}
	if uf.rank[rootX] < uf.rank[rootY] {
		rootX, rootY = rootY, rootX
	}
	uf.parent[rootY] = rootX
	uf.rank[rootX] += uf.rank[rootY]
	return true
}

func getNodeInDegrees(edges [][]int) []int {
	inDegrees := make([]int, len(edges)+1)
	for _, edge := range edges {
		inDegrees[edge[1]]++
	}
	return inDegrees
}

func isTree(edges [][]int, removeIndex int) bool {
	uf := NewUnionFind(len(edges) + 1)
	for i := 0; i < len(edges); i++ {
		if i == removeIndex {
			continue
		}
		if !uf.union(edges[i][0], edges[i][1]) {
			return false
		}
	}
	return true
}

func findRedundantDirectedConnection(edges [][]int) []int {
	ans := make([]int, 2)
	inDegrees := getNodeInDegrees(edges)
	for i := len(edges) - 1; i >= 0; i-- {
		if inDegrees[edges[i][1]] == 2 {
			if isTree(edges, i) {
				ans[0] = edges[i][0]
				ans[1] = edges[i][1]
				return ans
			}
		}
	}
	uf := NewUnionFind(len(edges) + 1)
	for _, edge := range edges {
		if !uf.union(edge[0], edge[1]) {
			ans[0] = edge[0]
			ans[1] = edge[1]
			return ans
		}
	}
	return ans
}