// Core tree data structure

#[derive(Debug, Clone)]
pub struct Node {
    pub name: String,
    pub is_directory: bool,
    pub children: Vec<Node>,
}

impl Node {
    pub fn new(name: String, is_directory: bool) -> Self {
        Self {
            name,
            is_directory,
            children: Vec::new(),
        }
    }
    
    pub fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }
}