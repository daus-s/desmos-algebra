module Main where

import LexDesmos
import ParDesmos
import AbsDesmos ()
import Interpreter


import ErrM
import PrintDesmos

main = do
  interact calc
  putStrLn ""

calc s = 
  let Ok e = pExp (myLexer s) 
  in printTree (eval e)
