pub struct UnionFind {
    pub parent: Vec<usize>,
    pub rank: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    pub fn find(&mut self, mut x: usize) -> usize {
        while self.parent[x] != x {
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }
        self.parent[x]
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let x = self.find(x);
        let y = self.find(y);
        if self.rank[x] > self.rank[y] {
            self.rank[x] += self.rank[y] + 1;
            self.parent[y] = x;
        } else {
            self.rank[y] += self.rank[x] + 1;
            self.parent[x] = y;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_find() {
        let mut uf = UnionFind::new(10);

        uf.parent = vec![1, 1, 2, 4, 2, 3, 6, 7, 8, 9];

        assert_eq!(uf.find(0), 1);

        assert_eq!(uf.find(1), 1);
        assert_eq!(uf.find(4), 2);
    }

    #[test]
    fn test_union() {
        let mut uf = UnionFind::new(10);

        uf.union(1, 2);
        assert_eq!(uf.parent[1], 2);
        assert_eq!(uf.parent[2], 2);

        uf.union(3, 4);
        assert_eq!(uf.parent[3], 4);
        assert_eq!(uf.parent[4], 4);

        uf.union(1, 3);
        assert_eq!(uf.parent[1], 2);
        assert_eq!(uf.parent[2], 4);
        assert_eq!(uf.parent[3], 4);
        assert_eq!(uf.parent[4], 4);
    }
}
