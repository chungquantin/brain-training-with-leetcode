package tree

import (
	"fmt"
)

const (
	ALPHABET_SIZE = 26
)

type TrieNode struct {
	childrens map[string]*TrieNode
	endOfWord bool
	level     int32
}

type Trie struct {
	root *TrieNode
}

func NewTrieNode() *TrieNode {
	return &TrieNode{
		childrens: make(map[string]*TrieNode, ALPHABET_SIZE),
		endOfWord: false,
		level:     0,
	}
}

func NewTrie() *Trie {
	return &Trie{
		root: NewTrieNode(),
	}
}

func (trie *Trie) Insert(word string) {
	curNode := trie.root
	index := 1
	for _, c := range word {
		strChar := string(c)
		if curNode.childrens[strChar] == nil {
			curNode.childrens[strChar] = NewTrieNode()
		}
		curNode.level = int32(index)
		node := curNode.childrens[strChar]
		curNode = node
		index++
	}
	curNode.level = int32(len(word))
	curNode.endOfWord = true
}

func (trie *Trie) Search(word string) bool {
	curNode := trie.root
	for _, c := range word {
		strChar := string(c)
		if curNode.childrens[strChar] == nil {
			return false
		}
		node := curNode.childrens[strChar]
		curNode = node
	}
	return curNode.endOfWord
}

func RunDsTrieTest() {
	fmt.Println("Data Structure: Trie...")
	trie1 := NewTrie()
	listOfWords := []string{"hello", "helium", "world"}
	for _, word := range listOfWords {
		trie1.Insert(word)

		searchResult := trie1.Search(word)
		fmt.Println("Search Result for word '"+word+"': ", searchResult)
	}
}
