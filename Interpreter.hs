{-# OPTIONS_GHC -Wno-incomplete-patterns #-}
module Interpreter where

import AbsDesmos
import Prelude
import Factorial

eval :: Exp -> Double
eval (Dbl d) = d
eval (Int i) = fromIntegral i
eval Pi = pi
eval E = 2.71828182845

eval (Plus i j) = eval i + eval j
eval (Minus i j) = eval i - eval j
eval (Times i j) = eval i * eval j
eval (Div i j) = eval i / eval j
eval (Negate i) = -1 * eval i
eval (Fact i) = fromIntegral (factorial (eval i))
eval (Exp i j) = eval i**eval j
eval (Sqrt i) = sqrt (eval i)
eval (NRt i j) = eval j**(1.0/ eval i)

eval (Ln i) = log (eval i)
eval (Log i) = logBase 10 (eval i)

--we will only use radians
--potentially a  mode will be added buyt for now radians 
eval (Sin i) = sin (eval i)
eval (Cos i) = cos (eval i)
eval (Tan i) = tan (eval i)

eval (ASin i) = asin (eval i)
eval (ACos i) = acos (eval i)
eval (ATan i) = atan (eval i)


---Misceallenaous Functions
eval (Abs i) = abs (eval i)
eval (Round i) = fromIntegral (round (eval i))
eval (Percent i j) = eval i / 100.0 * eval j