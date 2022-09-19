TODO
 - Reconsier all of the Option<> return values in functions that take node and edge ids. It might be better to panic on invalid ids instead, and document "valid ids" as a precondition of using the library
 - Implement node containers, edge containers, and adj containers that you can mix and match to plug into a single Graph struct to achieve different time complexities and functionality
 - For containers like Vec<Node>, mark with an Ordinal marker trait. When AdjContainer is Ordinal, require NodeContainer to be ordinal also
 - Containers that are not Ordinal can be marked with Keyed

Future functionality:
- Max flow
- Graph visualization
