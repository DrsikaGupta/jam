use std::collections::HashMap;

#[derive(Default)]
pub struct TrieNode {
    children: HashMap<char, TrieNode>,
    terminal: bool,
}

pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            root: TrieNode::default(),
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;

        for c in word.chars() {
            node = node.children.entry(c).or_default();
        }

        node.terminal = true;
    }

    pub fn starts_with(&self, prefix: &str) -> Vec<String> {
        let mut node = &self.root;

        for c in prefix.chars() {
            match node.children.get(&c) {
                Some(next) => node = next,
                None => return vec![],
            }
        }

        let mut out = Vec::new();

        Self::dfs(node, prefix.to_string(), &mut out);

        out
    }

    fn dfs(node: &TrieNode, current: String, out: &mut Vec<String>) {
        if node.terminal {
            out.push(current.clone());
        }

        for (c, child) in &node.children {
            let mut next = current.clone();
            next.push(*c);

            Self::dfs(child, next, out);
        }
    }
}
