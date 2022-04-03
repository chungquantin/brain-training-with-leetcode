# Graph Theory

## Disjoint Set

Disjoin set is a data structure used in graph algorithms to track the record of vertices and trace back to its root vertex. By that way, we can check the connectivity of vertices. 

*From this given graph*
```
Set of vertices

(0, 1) (0, 2) (1, 3) (4, 8) (5, 6) (5, 7)
```
```
Graph visualizaton

  0             4               5-6
 / \            |               |
1   2           8               7
|
3                        9
```
*Convert it to the disjoint set*

```
Array value: [0,0,0,1,4,5,5,5,4,9]
Array index: [0,1,2,3,4,5,6,7,8,9]
```

Two most important methods of the Disjoint Set are `find()` and `union()`

- ***Implementation with Quick Find***: in this case, the time complexity of the find function will be `O(1)`. However, the union function will take more time with the time complexity of `O(N)`.
- ***Implementation with Quick Union***: compared with the Quick Find implementation, the time complexity of the union function is better. Meanwhile, the find function will take more time in this case.

## Breadth First Search (BFS)

BFS is an algorithm to traverse the vertices in graph or the matrix. Like its name, BFS traverses through each node in the same level before moving to the next level.
```
                          0             
                         / \            
                        1   2         
                        |
                        3 
```
Take the above tree as an example, the traversal order of BFS is `0 -> 1 -> 2 -> 3`

The implementation of BFS using `Queue (First In First Out)`. 

Explain more why we use `Queue`, we `push_back()` all nodes to the queue then `pop_front()`, while we pop front, we again push back the neighbor nodes of the visited nodes.

See leetcode `Question 994 - Rottening Oranges` for the use of BFS.

## Depth First Search (DFS)
Differnt from BFS, DFS traverse the vertices in the order of level, which means for every node it visits, it will travese deeply to the child nodes. 

```
                          0             
                         / \            
                        1   2         
                        |
                        3 
```
Take the above tree as an example, the traversal order of BFS is `0 -> 1 -> 3 -> 2`

The implementation of DFS using `Stack (Last In First Out)`

Explain more why we use `Stack`, we `push()` all nodes to the stack then `pop()` to ge the node and then traversing to the connected nodes. Then `push()` related nodes to the stack until there is no connected nodes of that root node left.

See leetcode `Question 733 - Flood Fill` for the use of DFS