# CGraph - composable graph

TODO
 - Refactor Traversal into Tree trait and Traversal supertrait, impl Tree for ShortestPathTree, etc.
    - Paths don't need dist in path because user can just call `shortest_path_tree.dist()`. So use existing `Path` struct
    - Maybe add `root()` function to Tree that gives either Node or NId
    - Refactor astar to call and return Pfs
    - Refactor astar to not require ShortestPath
    - Refactor ShortestPathTree to borrow graph like the traversals do
    - Maybe get rid of ShortestPathTree and instead create PriorityTree that gets converted from a Pfs? But how does that work with Bellman Ford?
 - Add priority first search traversal that uses priority queue with custom priority. Replace dijkstra's implementation with pfs with min comparator
 - Better error messages: derive Debug for all data and format node ids, etc into errors
 - Maybe change the `insert_` functions that return Result into `try_insert_` and have the normal ones assert ids exist
 - Perf tests for dinic and other algos, maybe use competitive programming problem test cases
 - Add `bfs_where` that bfs's based on conditional function and `bfs_to` that searches for a specific target, and a function that does both (name tbd). Same for dfs.
 - Add `.path(target)` to bfs and dfs structs that returns path to a specific target, much like ShortestPathTree. Maybe find a way to reuse that code. Do this instead of `dfs_to`
    - Implement `.path()` function (maybe `into_path()`) on the iterator object that will call `next()` on the iterator until target is found and packages findings in a nice struct.
    - Maybe put this function in Traversal trait that subtraits Iterator and have Bfs and Dfs implement it
 - Fix ShortestPath to have Edge and Node instead of Edge and NodeId
 - Improve errors by including details (node/edge ids, etc) in error messages
 - Migrate some existing return types from Option<> to Result<>, especially those that are supposed to mutate the graph. Consider where to keep Option<>, like perhaps for `node()` and `edge()`
 - Collapse Edge and EdgeMut into a single struct where the generic type for data is either `&'a E` or `&'a mut E`?
 - Move `_mut()` iteration methods to their own trait, e.g. GraphMut
 - Add `between_multi_mut()`.
 - implement MultiGraph for CGraph and MultiAdj for relevant adj containers
 - Possible refactor - get rid of all these separate Keyed and Ordinal traits for containers and such, just assume everything is ordinal with usize ids. Create a wrapper struct KeyedGraph that maintains a map of keys to ordinals and an underlying ordinal graph, and provides a `put_node` method.
 - Reconsider all of the Option<> return values in functions that take node and edge ids. It might be better to panic on invalid ids instead, and document "valid ids" as a precondition of using the library
 - For containers like Vec<Node>, mark with an Ordinal marker trait. When AdjContainer is Ordinal, require NodeContainer to be ordinal also
 - Containers that are not Ordinal can be marked with Keyed
 - Idea for graph impl: Store edges in single list sorted by (u, v). Adjacency is tracked by storing for each node the start index of edges for that node. The DiGraph version can have a separate list for in edges. Insertion / removal is O(N+E), but iteration is very fast since it's just one vec. Call it FlatAdjList / FlatGraph
 - For containers, panic on invalid input. Invariants should be enforced at higher level (in CGraph). Clean up container contracts / spell out behavior assumptions explicitly. e.g. adj container `insert_node` has no return value. it should either return Option<> to denote error or panic on any invalid input
 - Max flow:
    - Create FlowGraph struct that holds G: DirectedGraph and implements Graph, but does custom logic for managing back edges and such, and provides special methods like `back_edges()`.
    - G::E must be type Flow, which is a struct holding some integer type storing capacity and flow and provides a method for `residual()`, or `cap - flow`
	- Forward edges start with 0 flow and N capacity, and saturate at N flow and N capacity. Back edges start with 0 flow and 0 capacity, and saturate at -N flow and 0 capacity
    - insert normal edges and back edges one after another so back edge can be determined by parity (`edge_id ^ 1`)
    - If FlowGraph holds a CGraph, ideally it should use a raw adjacency container and not Di<> because that would inefficiently store in edges separately even though the flow graph will do that anyway. Refactor the traits (DirectedGraph, DirectedAdjContainer) to let you make a DirectedGraph without using Di<>. One option is to have a marker trait for RawAdjContainer and impl DirectedGraph for graphs where AC: RawAdjContainer. This is more accurate anyway because raw adj containers are directed.
    - Better option, only allow FlowGraph to hold CGraph where AC: RawAdjContainer, which we know is directed but we don't need to wastefully implement an inefficient version of DirectedGraph.
    - `in_edges` can be calculated by just reversing the edge id parity of every out edge
 - Add reindex() operation for stable containers that gets rid of empty slots and returns a map of old indices to new indices
 - Adj containers to implement:
    - Vec<Map>
    - FlatGraph

Future functionality:
- Max flow
- Graph visualization
