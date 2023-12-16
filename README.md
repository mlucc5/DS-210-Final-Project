# DS-210-Final-Project
DS 210 Final Project, calculating the distances between cellular towers, connecting them by the shortest distance.
<br>
Here is a link to the dataset I used: https://hifld-geoplatform.opendata.arcgis.com/datasets/cellular-towers/explore?location=41.362709%2C-72.899753%2C9.01
<br>
I altered the original data by only including cell towers on the contiguous United States, unfortunately I cannot attach my Excel sheet onto this as the file is too big.
<br> <br>
This is the "write-up" portion of the final exam where I will explain my project, including what data I am using, what I am trying to solve, roadblocks, and how I overcame them.
<br> <br>
I used a dataset of 23312 nodes, after removing Cell Towers outside of the contiguous United States. The Excel spreadsheet logs every cellular tower in the United States, tracking various numbers that correspond to the location and differentiation of each tower. The values I use from the Excel spreadsheet in my code are longitude, latitude and the StateID which signals which state each node is placed in. I grabbed index 0, 1 and 18 respectively to access these values.
<br> <br>
The StateID is significant because I wanted to calculate the best path of connectivity in each state as to not overwhelm my computer. When I tried to connect the entire US at once, the code took too long to run. I have returned several states in order to show how my code works. It returns the state, the number of Cell Towers (vertexs/nodes), and number of Connections (edges). The number of connections returned was always one less than the number of cell towers. This was verified by a test I wrote into my code. The number of connections must be one less than the number of cell towers to satisfy a minimum spanning tree. A minimum spanning tree is one where each node is connected using the most efficient route. In other words, my code returns one big graph, fully traversible, where distance is weighted using the haversine function I moved to a module. Distance being weighted means each edge is the shortest connection that could be made. The resultant graph is the most efficient way to connect all nodes to one another.
<br> <br>
I included New England states and Tri-State states to show how grouped states can be calculated and shown off. An added bonus of my code is that it outputs the amount of nodes in each state you want. You can alter the implementation to return different variations and groups of states, this doesn't affect the returned total as well. Despite CT being in both Tri-State and NE, it is not counted twice.
<br> <br>
My code has one test and one module to allow for easier implementation. I chose haversine to be the module because the logic in that function is never altered when changing the code, while other logic is changed when I want to do something different. My test is necessary because it is an easy way to check if the connections are correct and if MST works properly.
<br> <br>
Finally I created a function to combine NY and CT. It could be any two states that border eachother, but it links the states together using MST to have one less connection than total nodes. Not only does my code connect nodes within a state but it also has the functionality to connect all the nodes in bordering states, in the most efficient manor. Once again, full US connection is not viable due to the amount of nodes but this is an exemplify how it is possible.
<br> <br>
There is only one module called hvs.rs and the dependencies I use in my toml file are petgraph = "0.6" and calamine = "0.17.0".
<br> <br>
Below is what the function produces:
<br> <br>
State: CT
<br>
Number of Cell Towers: 114
<br>
Number of Connections: 113
<br>
State: NJ
<br>
Number of Cell Towers: 170
<br>
Number of Connections: 169
<br>
State: MA
<br>
Number of Cell Towers: 143
<br>
Number of Connections: 142
<br>
State: VT
<br>
Number of Cell Towers: 68
<br>
Number of Connections: 67
<br>
State: ME
<br>
Number of Cell Towers: 212
<br>
Number of Connections: 211
<br>
State: NH
<br>
Number of Cell Towers: 145
<br>
Number of Connections: 144
<br>
State: RI
<br>
Number of Cell Towers: 29
<br>
Number of Connections: 28
<br>
State: NY
<br>
Number of Cell Towers: 636
<br>
Number of Connections: 635
<br>
Total Cell Towers: 1517
<br>
Total Cell Towers in NE: 711
<br>
Total Cell Towers in Tri-State: 920
<br>
Combined States
<br>
Total Cell Towers: 750
<br>
Number of Connections: 749
