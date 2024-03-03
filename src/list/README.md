# Tim sort

1. divide into run (len(run) => 32)
2. sort each run using insertion sort
3. insert each run into stack
4. the size of run is decreasing by going up the stack
5. the sum of the size of the two runs above a run is smaller than the size of the run
6. if 4, 5 not satisfied, merge the middle run with the smaller run
7. 