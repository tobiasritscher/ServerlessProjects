# Questions:

## How much time was spent on each individual function? 
  
  f0: 1.131928s
  
  f1: 0.0208880000000002s
  
  f2: 0.1804500000000003s
  
  f3: 0.220811s
  
  f4: 0.0231240000000001s

## If an error occurs, which function failed and why? 
Non error occured.

## Can you provoke a coldstart situation where the pre-warming shows actual effects? 
This is nearly impossible, because we can't decide where the function should run. So it's totaly random if google executes the function on a warm or a cold processor.
With the warmup call we can just make sure, that a cold processor is ready for our real function.
  
## Are conditionally branched functions invoked less? 
Yes, because only one of the conditional function will be called at once.
  
## Is prewarming more helpful with longer sequences? 
Yes, except when runtime is longer than cache time.

## Did your workflow manager function ever time out?
Never.
