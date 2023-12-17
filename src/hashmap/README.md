# HASHMAP

## Collision Resolution Method

 - Open addressing
   - Linear probing
     - if h(x) collides, try h(x) + 1, h(x) + 2, ...
     - The clustered keys called cluster
     - Searching: h(x) -> if found => return else => (h(x) + 1) -> if found => return else => (h(x) + 2) -> ..., if h(x) +i doesn't exist -> return not found
   - Quadratic probing
   - Double hashing
 - Chaining
   - 
