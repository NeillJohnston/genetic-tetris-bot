# genetic-tetris-bot

This is my second take on making a Tetris bot. My [first attempt](https://github.com/NeillJohnston/classic-tetris-bot) was a simple heuristic algorithm that I tweaked manually and plugged into an emulator. This takes that idea a step further by applying a genetic algorithm to tweak the heuristics, creating a much better bot.

This repo has 3 separate crates in it: 1. the Tetris utility library in /tetris, 2. the genetic algorithm binary in /genetic, and 3. the socket-based emulator interface in /interface.
