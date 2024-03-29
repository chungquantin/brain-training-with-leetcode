var criticalConnections = function(n, connections) {
  const adjacency = {}
  for (let i = 0; i < n; i++) {
    adjacency[i] = []
  }

  for (let [node1, node2] of connections) {
    adjacency[node1].push(node2)
    adjacency[node2].push(node1)
  }

  const dfsNumber = new Array(n).fill(0)
  // this is what we called oldestReachable
  // the lowest number reachable by dfs from a node
  const dfsLow = new Array(n).fill(0)
  const criticalEdges = []
  let timestamp = 0

  function tarjan(node, parent, adjacency, edges) {
    timestamp++
    dfsNumber[node] = timestamp
    dfsLow[node] = timestamp
    for (let neighbor of adjacency[node]) {
      if (neighbor === parent) continue
      if (!dfsNumber[neighbor]) tarjan(neighbor, node, adjacency, edges)

      // resetting oldestReachable if neighbor can cycle back
      dfsLow[node] = Math.min(dfsLow[node], dfsLow[neighbor])

      // if neighbor cannot reach back to me or oldest, we have a critical edge
      if (dfsLow[neighbor] > dfsNumber[node]) edges.push([node, neighbor])
    }
  }

  tarjan(0, null, adjacency, criticalEdges)

  return criticalEdges
}

console.log(criticalConnections(5, [[1,0],[2,0],[3,2],[4,2],[4,3],[3,0],[4,0]]))