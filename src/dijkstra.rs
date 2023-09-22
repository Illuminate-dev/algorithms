use crate::trees::heap::MaxHeap;

fn find_shortest_path(connections: &[(usize, usize, i32)], from: usize, to: usize, n: usize) -> Option<i32> {
    let mut adj = vec![Vec::new(); n];
    for (f, t, w) in connections {
        adj[*f].push((t, w));
    }

    let mut h = MaxHeap::new();
    h.push((0, from));

    while !h.is_empty() {
        let Some((w, n)) = h.pop_max() else { unreachable!() }; 

        if n == to {
            return Some(w);
        }

        for (t, w2) in &adj[n] {
            h.push((w+**w2, **t));
        }

    
    }

    None

}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test_find_shortest_path() {
        let x = find_shortest_path(&[(0, 1, 3), (1, 2, 3), (0, 2, 4)], 0, 2, 3);
        assert_eq!(x, Some(4));
    }
}