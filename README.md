 TODO
- Implement BFS and DFS as structs implementing Iterator, and add `.bfs()`, `.bfs_from()`, `.dfs()`, `.dfs_from()` functions so we don't have to use Bfs::new() and Dfs::new() every time
- Change Graph trait to support multi-edge graph cases and undirected edges
- Wrap return values in structs that implement iterators over whatever types we want (e.g. Edges struct that allows you to iterate over (u, v, w))
- Wrap edges in Edge struct that stores ingoing/outgoing, direct/undirected, u, v, etc
- Split Graph trait into multiple traits depending on implementation. Core things like iteration and getters should be in the Graph trait but mutation (insert/delete/update) and maybe initialization (maybe shouldn't be in a trait at all) should be handled in other traits. For example, graphs that allow users to specify a node index might have `fn insert_node(idx, node)` but other graphs might have `fn insert_node(node) -> idx`
