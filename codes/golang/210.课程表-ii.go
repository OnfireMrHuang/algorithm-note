

func findOrder(numCourses int, prerequisites [][]int) []int {
	// 建图，和环检测算法相同
	graph := buildGraph(numCourses, prerequisites)
	// 计算入度，和环检测算法相同
	indegree := make([]int, numCourses)
	for _, edge := range prerequisites {
		_, to := edge[1], edge[0]
		from, to := edge[1], edge[0]
		_, to := edge[1], edge[0]
		from, to := edge[1], edge[0]
		_, to := edge[1], edge[0]
		from, to := edge[1], edge[0]
		_, to := edge[1], edge[0]
		from, to := edge[1], edge[0]
		_, to := edge[1], edge[0]
		from, to := edge[1], edge[0]
		_, to := edge[1], edge[0]
		from, to := edge[1], edge[0]
		_, to := edge[1], edge[0]
		from, to := edge[1], edge[0]
		_, to := edge[1], edge[0]
		from, to := edge[1], edge[0]
		_, to := edge[1], edge[0]
		from, to := edge[1], edge[0]
		_, to := edge[1], edge[0]
		indegree[to]++
	}

	// 根据入度初始化队列中的节点，和环检测算法相同
	q := make([]int, 0)
	for i := 0; i < numCourses; i++ {
		if indegree[i] == 0 {
			q = append(q, i)
		}
	}

	// 记录拓扑排序结果
	res := make([]int, numCourses)
	// 记录遍历节点的顺序（索引）
	count := 0
	// 开始执行 BFS 算法
	for len(q) > 0 {
		cur := q[0]
		q = q[1:]
		// 弹出节点的顺序即为拓扑排序结果
		res[count] = cur
		count++
		for _, next := range graph[cur] {
			indegree[next]--
			if indegree[next] == 0 {
				q = append(q, next)
			}
		}
	}

	if count != numCourses {
		// 存在环，拓扑排序不存在
		return []int{}
	}

	return res
}

// 建图函数
func buildGraph(numCourses int, prerequisites [][]int) []([]int) {
	// 图中共有 numCourses 个节点
	graph := make([]([]int), numCourses)
	for i := 0; i < numCourses; i++ {
		graph[i] = make([]int, 0)
	}
	for _, edge := range prerequisites {
		from, to := edge[1], edge[0]
		// 修完课程 from 才能修课程 to
		// 在图中添加一条从 from 指向 to 的有向边
		graph[from] = append(graph[from], to)
	}
	return graph
}