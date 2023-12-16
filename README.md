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
<br><br>
My code has one test and one module to allow for easier implementation. I chose haversine to be the module because the logic in that function is never altered when changing the code, while other logic is changed when I want to do something different. My test is necessary because it is an easy way to check if the connections are correct and if MST works properly.
