# snake.py

from collections import deque
import torch
from torch import nn
import gymnasium as gym
import numpy as np
import random

class DQN(nn.Module):
    def __init__(self, in_states, h1_nodes, out_actions):
        super.__init__()

        # define layers 
        self.fc1 = nn.Linear(in_states, h1_nodes)   # input layer
        self.out = nn.Linear(h1_nodes, out_actions) # output layer
        
        
# import math
# pi = 3.1415926536
# print(math.cos((3/2)*pi))