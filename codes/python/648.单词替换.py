class Trie:
    def __init__(self):
        self.children = [None] * 26
        self.isEnd = False

    def insert(self, word: str):
        node = self
        for ch in word:
            if not node.children[ord(ch) - ord('a')]:
                node.children[ord(ch) - ord('a')] = Trie()
            node = node.children[ord(ch) - ord('a')]
        node.isEnd = True

class Solution:
    def replaceWords(self, dictionary: List[str], sentence: str) -> str:
        trie = Trie()
        for root in dictionary:
            trie.insert(root)
        
        ans = []
        words = sentence.split()
        for word in words:
            node = trie
            find_prefix = False
            prefix = ""
            for ch in word:
                if node.children[ord(ch) - ord('a')]:
                    prefix += ch
                    if node.children[ord(ch) - ord('a')].isEnd:
                        find_prefix = True
                        break
                    node = node.children[ord(ch) - ord('a')]
                else:
                    break
            if find_prefix:
                ans.append(prefix)
            else:
                ans.append(word)
        return " ".join(ans)
        