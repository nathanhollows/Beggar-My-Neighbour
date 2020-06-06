# Beggar My Neighbour

This is my attempt to implement genetic algorithms in order to get a new longest sequence of cards in the game [Beggar My Neighbour](https://en.wikipedia.org/wiki/Beggar-my-neighbour)

## Why?

Two reasons: Firstly, I want to learn how to implement genetic algorithms. Secondly, I want to get into the [leaderboard](http://www.richardpmann.com/beggar-my-neighbour-records.html)

## Update

Turns out genetic algorithms are bad for this. The smallest changes in the ordering of cards massively affects the outcome - usually these changes are negative. Mutation and cross over of cards makes the overall population move away from the target.

I also rewrote the project in rust, for faster brute forcing.
