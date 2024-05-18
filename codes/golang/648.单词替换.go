

type Trie struct {
	children map[rune]*Trie
	isEnd    bool
}

func NewTrie() *Trie {
	return &Trie{
		children: make(map[rune]*Trie),
		isEnd:    false,
	}
}

func (t *Trie) insert(word string) {
	node := t
	for _, ch := range word {
		if node.children[ch] == nil {
			node.children[ch] = NewTrie()
		}
		node = node.children[ch]
	}
	node.isEnd = true
}

func replaceWords(dictionary []string, sentence string) string {
	trie := NewTrie()
	for _, root := range dictionary {
		trie.insert(root)
	}

	var ans strings.Builder
	words := strings.Split(sentence, " ")
	for _, word := range words {
		node := trie
		var prefix strings.Builder
		var find_prefix bool
		for _, ch := range word {
			if node.children[ch] == nil {
				break
			}
			prefix.WriteRune(ch)
			if node.children[ch].isEnd {
				find_prefix = true
				break
			}
			node = node.children[ch]
		}
		if find_prefix {
			ans.WriteString(prefix.String())
		} else {
			ans.WriteString(word)
		}
		ans.WriteString(" ")
	}
	return strings.TrimRight(ans.String(), " ")
}