#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
    liberties: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
            liberties: vec![4; n], // initial liberties set to 4 for simplicity
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x != root_y {
            if self.size[root_x] < self.size[root_y] {
                self.parent[root_x] = root_y;
                self.size[root_y] += self.size[root_x];
                self.liberties[root_y] += self.liberties[root_x];
            } else {
                self.parent[root_y] = root_x;
                self.size[root_x] += self.size[root_y];
                self.liberties[root_x] += self.liberties[root_y];
            }
        }
    }

    pub fn subtract_liberty(&mut self, x: usize) {
        let root = self.find(x);
        if self.liberties[root] > 0 {
            self.liberties[root] -= 1;
        }
    }

    pub fn add_liberty(&mut self, x: usize, n: isize) {
        let root = self.find(x);
        self.liberties[root] = self.liberties[root].saturating_add_signed(n);
    }

    pub fn get_liberties(&mut self, x: usize) -> usize {
        let root = self.find(x);
        self.liberties[root]
    }

    pub fn reset_node(&mut self, x: usize) {
        self.parent[x] = x; // Reset to point to itself
        self.size[x] = 1;
        self.liberties[x] = 4; // Reset liberties to initial value
    }
}
