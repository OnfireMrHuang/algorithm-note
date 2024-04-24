
type UF struct {
	// 记录连通分量个数
	Count int
	// 存储若干棵树
	Parent []int
	// 记录树的“重量”
	Size []int
}

func NewUF(n int) *UF {
	u := UF{}
	u.Count = n
	u.Parent = make([]int, n)
	u.Size = make([]int, n)
	for i := 0; i < n; i++ {
		u.Parent[i] = i
		u.Size[i] = 1
	}
	return &u
}

/* 将 p 和 q 连通 */
func (u *UF) Union(p, q int) {
	rootP := u.find(p)
	rootQ := u.find(q)
	if rootP == rootQ {
		return
	}

	// 小树接到大树下面，较平衡
	if u.Size[rootP] > u.Size[rootQ] {
		u.Parent[rootQ] = rootP
		u.Size[rootP] += u.Size[rootQ]
	} else {
		u.Parent[rootP] = rootQ
		u.Size[rootQ] += u.Size[rootP]
	}
	u.Count--
}

/* 判断 p 和 q 是否互相连通 */
func (u *UF) Connected(p, q int) bool {
	rootP := u.find(p)
	rootQ := u.find(q)
	// 处于同一棵树上的节点，相互连通
	return rootP == rootQ
}

/* 返回节点 x 的根节点 */
func (u *UF) find(x int) int {
	for u.Parent[x] != x {
		// 进行路径压缩
		u.Parent[x] = u.Parent[u.Parent[x]]
		x = u.Parent[x]
	}
	return x
}

func equationsPossible(equations []string) bool {
	// 26 个英文字母
	uf := NewUF(26)
	// 先让相等的字母形成连通分量
	for _, eq := range equations {
		if eq[1] == '=' {
			x := eq[0]
			y := eq[3]
			uf.Union(int(x-'a'), int(y-'a'))
		}
	}
	// 检查不等关系是否打破相等关系的连通性
	for _, eq := range equations {
		if eq[1] == '!' {
			x := eq[0]
			y := eq[3]
			// 如果相等关系成立，就是逻辑冲突
			if uf.Connected(int(x-'a'), int(y-'a')) {
				return false
			}
		}
	}
	return true
}