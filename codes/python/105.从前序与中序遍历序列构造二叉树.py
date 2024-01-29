
class Solution:
    def buildTree(self, preorder: List[int], inorder: List[int]) -> TreeNode:
        # 首先：针对中序遍历构造一个映射
        inorder_map = {}
        for i, v in enumerate(inorder):
            inorder_map[v] = i
        # 接着，通过前序遍历数组和中序映射开始递归构造二叉树
        return self.recursion_build(preorder, 0, len(preorder)-1, inorder, 0, len(inorder)-1, inorder_map)
            
    def recursion_build(self,preorder: List[int], pre_start: int, pre_end:int, 
                        inorder: List[int],in_start:int, in_end: int,inorder_map: dict):
        # 首先，判断一下递归终止条件
        if pre_start > pre_end:
            return None
        # 构造根节点
        root_val = preorder[pre_start]
        root = TreeNode(root_val)
        # 从中序遍历映射中找到索引
        root_inorder_index = inorder_map[root_val]
        # 计算左子树长度
        left_tree_len = root_inorder_index - in_start
        # 计算右子树长度
        right_tree_len = in_end - root_inorder_index
        # 构造左子树
        root.left = self.recursion_build(preorder, pre_start+1, pre_start+left_tree_len, inorder, in_start, root_inorder_index-1, inorder_map)
        # 构造右子树
        root.right = self.recursion_build(preorder, pre_end-right_tree_len+1, pre_end, inorder, root_inorder_index+1, in_end, inorder_map)
        return root