# HASHMAP

## Collision Resolution Method

 - Open addressing
   - If hashmap is maintained with total length is greater than the n(n can be float) times of number of keys, then cluster average size is O(1) 
     - Linear probing
       - if h(x) collides, try h(x) + 1, h(x) + 2, ...
       - The clustered keys called cluster
       - Searching: h(x) -> if found => return else => (h(x) + 1) -> if found => return else => (h(x) + 2) -> ..., if h(x) +i doesn't exist -> return not found
     - Quadratic probing
       - if h(x) collides, try h(x) + 1^2, h(x) + 2^2, ...
     - Double hashing
       - if h(x) collides, try h(x) + 1 * g(x), h(x) + 3 * g(x), ...
 - Chaining
   - 
