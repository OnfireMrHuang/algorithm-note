/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */

type Codec struct {
}

func Constructor() Codec {
	return Codec{}
}

// Serializes a tree to a single string.
func (this *Codec) serialize(root *TreeNode) string {
	if root == nil {
		return "null"
	}
	var root_val = strconv.Itoa(root.Val)
	var left = this.serialize(root.Left)
	var right = this.serialize(root.Right)
	return root_val + "," + left + "," + right
}

// Deserializes your encoded data to tree.
func (this *Codec) deserialize(data string) *TreeNode {
	var data_list = strings.Split(data, ",")
	return this.deserialize_helper(&data_list)
}

func (this *Codec) deserialize_helper(data *[]string) *TreeNode {
	if (*data)[0] == "null" {
		*data = (*data)[1:]
		return nil
	}
	var root_val, _ = strconv.Atoi((*data)[0])
	*data = (*data)[1:]
	var root = &TreeNode{Val: root_val}
	root.Left = this.deserialize_helper(data)
	root.Right = this.deserialize_helper(data)
	return root
}

/**
 * Your Codec object will be instantiated and called as such:
 * ser := Constructor()
 * deser := Constructor()
 * tree := ser.serialize(root)
 * ans := deser.deserialize(tree)
 * return ans
 */