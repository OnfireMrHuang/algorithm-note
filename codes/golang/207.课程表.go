

func canFinish(numCourses int, prerequisites [][]int) bool {
	// 记录一次 traverse 递归经过的节点
	onPath := make([]bool, numCourses)
	// 记录遍历过的节点，防止走回头路
	visited := make([]bool, numCourses)
	// 记录图中是否有环
	hasCycle := false

	graph := buildGraph(numCourses, prerequisites)

	for i := 0; i < numCourses; i++ {
		// 遍历图中的所有节点
		traverse(graph, i, &hasCycle, visited, onPath)
	}
	// 只要没有循环依赖可以完成所有课程
	return !hasCycle
}

func traverse(graph []LinkedList, s int, hasCycle *bool, visited, onPath []bool) {
	if onPath[s] {
		// 出现环
		*hasCycle = true
	}

	if visited[s] || *hasCycle {
		// 如果已经找到了环，也不用再遍历了
		return
	}
	// 前序遍历代码位置
	visited[s] = true
	onPath[s] = true
	for _, t := range graph[s].list {
		traverse(graph, t, hasCycle, visited, onPath)
	}
	// 后序遍历代码位置
	onPath[s] = false
}

type LinkedList struct {
	list []int
}

func buildGraph(numCourses int, prerequisites [][]int) []LinkedList {
	// 图中共有 numCourses 个节点
	graph := make([]LinkedList, numCourses)
	for i := 0; i < numCourses; i++ {
		graph[i] = LinkedList{list: []int{}}
	}
	for _, edge := range prerequisites {
		from := edge[1]
		to := edge[0]
		// 修完课程 from 才能修课程 to
		// 在图中添加一条从 from 指向 to 的有向边
		graph[from].list = append(graph[from].list, to)
	}
	return graph
}