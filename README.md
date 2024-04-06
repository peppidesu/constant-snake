# constant-snake

A constant-time per iteration version of snake, written in Rust.

## What do you mean by 'constant-time'?

Each iteration after the first one, that is, everything regarding game-logic and 
drawing stuff onto the terminal, is done in O(1) time at best (unless there are 
for-loops in the standard library functions). That is, it does not scale with 
the length of the snake, the size of the playing field, etc. There is only one 
exception to  this, which is the algorithm of finding a new position for the 
apple. There is a  way to make this constant-time too, but I haven't figured
this out yet. So long as you don't reach end-game, this part is basically 
constant-time.

## How it works

Every time the snake moves, you could argue that only the tail and the head of 
the snake change. more specifically, you could think of "moving forward" as
moving the tail piece in front of the head. this can be done in O(1) time using
a linked list. A special implementation of a linked list (CellBuf) is used to 
make sure all body pieces are adjecent in memory, reducing cache misses.

For collision detection, a bitmask is used. Because once again only the head and
tail pieces change visually, there are at most only two bits that need to be set 
in the mask, making this an O(1) operation as well.

We draw the entire screen at the beginning of the game, which requires some for 
loops for the borders (although one could remove the borders to make the game 
entirely constant-time, but i felt like that would take away from the 
playability aspect a bit too much). After this, we only re-draw the parts of the
snake that change, and only re-draw the apple when it gets eaten. All of this is
of course O(1).

## How to run

### Requirements
- `rustc >=1.76.0`

### Steps
1. Clone the repo with:
    ```
    $ git clone git@github.com:peppidesu/constant-snake
    ```

2. cd into `constant-snake` and run:
   ```
   $ cargo run
   ``` 


### How to play
- Use the arrow keys to move. 
- Exit with `Esc`.