module Factorial where

factorial :: Double -> Int
factorial 0 = 1
factorial n = round n * factorial (n-1)