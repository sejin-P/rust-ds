# Persistent Data Structure

## Partial Persistent Data Structure

Partially persistent data structure is a data structure that allows all versions of the data structure to be accessed, but only the latest version can be modified.

### Statement

Any kind of pointer-machine data structure, which contains constant(<=p) number of pointers to any node(+ pointer to itself is constant, too) can be made partially persistent with O(1) space and O(1) amortized time overhead.

### Proof

Methods: Fat node, Node copying

#### Fat node

- Each node has back pointer, which points to the node which points to this node. (<=p)
- it has version history, which contains version number, updated filed, and updated value. But its length should be constant(<2p).


Read operation
- if the version is the latest, just read the value from the node.
- if the version is not the latest, read the value from the version history.


Update operation
- if version history is not full, just add the version history, and update the value.
- if version history is full, create new node which has empty version history and the last update operation applied.
- we should update the back pointer of the node which points to this node to the new node.
- it happens recursively because of above operation.

In amortized analysis, we can add cost to each update operation to pay the cost of creating new node.

Therefore, we can make any kind of pointer-machine data structure partially persistent with O(1) space and O(1) amortized time overhead.
