class Solution:
    def findOrder(self, numCourses: int, prerequisites: List[List[int]]) -> List[int]:
        # 首先构造图
        graph = build_graph(numCourses, prerequisites)

        # 然后开始初始化入度表
        in_degree = [0] * numCourses
        for courses in graph:
            for course in courses:
                in_degree[course] += 1

        # 选择入度为0的顶点开始广度优先遍历
        queue = []
        for i in range(numCourses):
            if in_degree[i] == 0:
                queue.append(i)

        res = []
        # 开始广度优先遍历
        while queue:
            course = queue.pop(0)
            res.append(course)
            for next_course in graph[course]:
                in_degree[next_course] -= 1
                if in_degree[next_course] == 0:
                    queue.append(next_course)

        # 如果结果列表的长度与总课程数相等，说明无环，那么返回结果； 否则有环返回空列表
        return res if len(res) == numCourses else []


def build_graph(numCourses: int, prerequisites: List[List[int]]) -> List[List[int]]:
    """
    构造图: 其中顶点是课程，边是依赖该课程的其他课程
    """
    graph = [[] for _ in range(numCourses)]
    for course, pre in prerequisites:
        graph[pre].append(course)
    return graph

