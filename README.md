# CGraph - composable graph

TODO
 - Move keys and reverse_keys into single Mapping struct for Keyed? Or maybe just use bimap crate?
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
