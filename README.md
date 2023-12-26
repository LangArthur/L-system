# L-system

A graphical representation of Lindenmayer system. More details about l-system [here](http://algorithmicbotany.org/papers/abop/abop.pdf).

## configuring the renderer

The program used a config file (config.yaml), at the root of the project. This file is composed by definition of different lsystems in the following format:

```yml
heather: # name of your system
  axiom: X # axiom or original state
  rules: # a list of rules to apply at each iteration (more information about the used grammar [here](https://en.wikipedia.org/wiki/L-system#Example_4:_Koch_curve))
    'X':
      - prob: 0.8 # probability for this production to happens
        production: F[+X]F[-X]+X
      - prob: 0.2
        production: F[-X]F[+X]+X
- prob: 1 
        production: F[+X]F[-X]+X
    'F':
      # if not probability is specified, 1 is the default
      - production: FF
  delta: 20.0 # the angle taken at each rotate symbol
```

## How to use

Running the program will open a window with one system from the config loaded, at its first step.
Please find all keys linked to there respective actions:

- Up / Down arrow: go to the next / previous step of the system.
- Right / Left arrow: load the next system present in the config. (Warning: it can takes some time to render higher steps, please be patient)
- Ctrl and + / -: increase / decrease the size of each segment (50 pixels by default).
- Alt and + / -: increase / decrease the angle at each intersection.
- R: reload the current system (useful when some random are present in the generation process)
