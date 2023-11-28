# CGraph - composable graph

TODO 
 - Optimizations:
    - Create an EmptyContainer that implements both NodeContainer and EdgeContainer to be used when the node or edge type is `()`
    - Consider calculating paths by getting all edge ids first and then resolving edge data / nodes afterwards, better cache locality
    - Figure out a scheme to store adjacency list edges in an order that optimizes cache locality, especially for a flat adj list. e.g. reorder the "nodes" (blocks of edges) so that adjacent nodes are next to each other.
 - Simplify user experience of building a graph by creating an Abstract Factory type of Builder, say "GraphBuilder", where you can specify all the variant details like keyed vs ordinal,
 flat vs adj list vs adj map, directed vs undirected vs flow, etc. 
    - Something like: GraphBuilder<N, E>::keyed<&str>().directed().flat().build()
    - Tree of builders to navigate this. i.e. GraphBuilder -> KeyedGraphBuilder -> DirectedGraphBuilder -> build()
    - could have defaults for containers, e.g. default to "flat" unless otherwise specified. This can
    - alternatively, the builder can explicitly take strategy enum values and infer the correct type. like, telling giving it FASTREADS vs FASTWRITES
    - In general, find a good implementation for the Strategy pattern with graphs
    - following the subsequent TODO, don't include keyed in the builder, everything is ordinal and users can optionally use the Keyed wrapper
    - Create builder for Flow graph that uses `insert_flow_edge` and such
 - Refactor - get rid of all these separate Keyed and Ordinal traits for containers and such, just assume everything is ordinal with usize ids. Use Keyed<> wrapper struct on arbitrary graph whenever needed.
 - Add some kind of postfix calculation option for a Dfs that simulates calculating something after a recursive call. This can be done by reading the iterator in reverse.
 - Refactor Traversal into Tree trait and Traversal supertrait, impl Tree for ShortestPathTree, etc.
    - Maybe add `root()` function to Tree that gives either Node or NId
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
