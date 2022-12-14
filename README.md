TODO
 - Reconsider all of the Option<> return values in functions that take node and edge ids. It might be better to panic on invalid ids instead, and document "valid ids" as a precondition of using the library
 - For containers like Vec<Node>, mark with an Ordinal marker trait. When AdjContainer is Ordinal, require NodeContainer to be ordinal also
 - Containers that are not Ordinal can be marked with Keyed
 - Idea for graph impl: Store edges in single list sorted by (u, v). Adjacency is tracked by storing for each node the start index of edges for that node. The DiGraph version can have a separate list for in edges. Insertion / removal is O(N+E), but iteration is very fast since it's just one vec. Call it FlatAdjList / FlatGraph
 - For containers, panic on invalid input. Invariants should be enforced at higher level (in CGraph). Clean up container contracts / spell out behavior assumptions explicitly.
 - Make all (NId, EId) / (EId, NId) tuples follow a single convention
 - Change old unit tests to use CGraph types and then deprecate old VecGraph and MapGraph
 - Add `with_capacity()`
 - Add From<> to CGraph
 - Max flow:
    - Create FlowGraph struct that holds G: DirectedGraph and implements Graph, but does custom logic for managing back edges and such, and provides special methods like `back_edges()`.
    - G::E must be type Flow, which is a struct holding some integer type storing capacity and flow and provides a method for `residual()`, or `cap - flow`
    - insert normal edges and back edges one after another so back edge can be determined by parity (`edge_id ^ 1`)
    - If FlowGraph holds a CGraph, ideally it should use a raw adjacency container and not Di<> because that would inefficiently store in edges separately even though the flow graph will do that anyway. Refactor the traits (DirectedGraph, DirectedAdjContainer) to let you make a DirectedGraph without using Di<>. One option is to have a marker trait for RawAdjContainer and impl DirectedGraph for graphs where AC: RawAdjContainer. This is more accurate anyway because raw adj containers are directed.
    - `in_edges` can be calculated by just reversing the edge id parity of every out edge

Future functionality:
- Max flow
- Graph visualization
