use std::boxed::Box;

#[derive(Debug)]
struct TrieNode {
 c_byte: u8,
 is_word: bool,
 children: [Option<Box<TrieNode>>; 26],
}

impl TrieNode {
 fn new(c_byte: u8) -> Self {
  TrieNode {
   c_byte,
   is_word: false,
   children: Default::default(),
  }
 }
}

#[derive(Debug)]
struct Trie {
 head: TrieNode,
}

impl Trie {
 fn new() -> Self {
  Trie {
   head: TrieNode::new(b'\0'),
  }
 }

 /** Inserts a word into the trie. */
 fn insert(&mut self, word: String) {
  let mut curr: &mut TrieNode = &mut self.head;
  for b in word.bytes() {
   let e_byte = Self::e_byte(b);
   let node = TrieNode::new(e_byte);
   curr = curr.children[e_byte as usize].get_or_insert(Box::new(node));
  }
  curr.is_word = true;
 }

 /** Returns if the word is in the trie. */
 fn search(&self, word: String) -> bool {
  match self.search_node(word) {
   Some(node) => node.is_word,
   None => false,
  }
 }

 /** Returns if there is any word in the trie that starts with the given prefix. */
 fn starts_with(&self, prefix: String) -> bool {
  self.search_node(prefix).is_some()
 }

 fn search_node(&self, prefix: String) -> Option<&TrieNode> {
  let mut curr: &TrieNode = &self.head;
  for b in prefix.bytes() {
   let e_byte = Self::e_byte(b);
   match &curr.children[e_byte as usize] {
    Some(node) => curr = node,
    None => return None,
   }
  }

  Some(curr)
 }

 fn e_byte(b: u8) -> u8 {
  return b - b'a';
 }
}
