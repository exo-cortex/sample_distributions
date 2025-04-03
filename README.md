# Creating unusual 2D probability distributions

This program is mainly an experiment to try and sample from 2D probability distributions.
The sampling process works as follows:
- Define a probability density function p(x,y).
- Pick a random candidate point C(x,y) and calculate the probability p(x,y).
- Pick a random value v between 0 and 1. If v < p(x,y) the candidate is successful and is being written into the file. Otherwise the candidate will be rejected.

Note: this method is typically yielding drastically less sample points than candidate points. It is not very efficient.

## Current Distribution shapes:
- Normal distribution
- Ring-shaped distribution

## Todo:
- finite Line-distribution - A line connecting 2 points