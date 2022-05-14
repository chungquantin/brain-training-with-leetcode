### Search algorithm

- Linear search
- Binary search

### Graph Traversal

- Breadth First Search
- Depth First Search

Problem with BFS and DFS is **combinatorial explosion**. Number of possible states of the puzzle is too hgih to be practical.

#### Blind search method

```
begin
procedure UniformCostSearch(Graph, root, goal)
  node:= root, cost = 0
  frontier:= priority queue containing node only
  explored:= empty set
  do
    if frontier is empty
      return failure
    node:= frontier.pop()
    if node is goal
      return solution
    explored.add(node)
    for each of node's neighbors n
      if n is not in explored
        if n is not in frontier
          frontier.add(n)
        else if n is in frontier with higher cost
          replace existing node with n
```

#### "Look forward" class search method

One class of heuristic search algorithms looks forward into the state-space graph. Whenever two or more alternative paths appear, these algorithms pursue the path or paths closest to the goal state.

- Hill climbing
- Beam search
- Best first search

### "Look backward" class search method

Another class of algorithms "look backward" and are referred to as branch and bound methods

- A\* algorithm (does some looking forward as well)
