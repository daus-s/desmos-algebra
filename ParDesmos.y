-- This Happy file was machine-generated by the BNF converter
{
{-# OPTIONS_GHC -fno-warn-incomplete-patterns -fno-warn-overlapping-patterns #-}
module ParDesmos
  ( happyError
  , myLexer
  , pExp1
  , pExp2
  , pExp3
  , pExp4
  , pExp5
  , pExp6
  , pExp7
  , pExp8
  , pExp9
  , pExp10
  , pExp
  , pExp11
  ) where

import Prelude

import qualified AbsDesmos
import LexDesmos

}

%name pExp1 Exp1
%name pExp2 Exp2
%name pExp3 Exp3
%name pExp4 Exp4
%name pExp5 Exp5
%name pExp6 Exp6
%name pExp7 Exp7
%name pExp8 Exp8
%name pExp9 Exp9
%name pExp10 Exp10
%name pExp Exp
%name pExp11 Exp11
-- no lexer declaration
%monad { Err } { (>>=) } { return }
%tokentype {Token}
%token
  '!' { PT _ (TS _ 1) }
  '(' { PT _ (TS _ 2) }
  ')' { PT _ (TS _ 3) }
  '+' { PT _ (TS _ 4) }
  ',' { PT _ (TS _ 5) }
  '-' { PT _ (TS _ 6) }
  '=' { PT _ (TS _ 7) }
  '[' { PT _ (TS _ 8) }
  '\\%\\operatorname{of}' { PT _ (TS _ 9) }
  '\\cdot' { PT _ (TS _ 10) }
  '\\cos\\left(' { PT _ (TS _ 11) }
  '\\cos^{-1}\\left(' { PT _ (TS _ 12) }
  '\\frac{' { PT _ (TS _ 13) }
  '\\left|' { PT _ (TS _ 14) }
  '\\ln' { PT _ (TS _ 15) }
  '\\log' { PT _ (TS _ 16) }
  '\\operatorname{mean}\\left(' { PT _ (TS _ 17) }
  '\\operatorname{nCr}\\left(' { PT _ (TS _ 18) }
  '\\operatorname{nPr}\\left(' { PT _ (TS _ 19) }
  '\\operatorname{round}\\left(' { PT _ (TS _ 20) }
  '\\operatorname{stdevp}\\left(' { PT _ (TS _ 21) }
  '\\operatorname{stdev}\\left(' { PT _ (TS _ 22) }
  '\\pi' { PT _ (TS _ 23) }
  '\\right)' { PT _ (TS _ 24) }
  '\\right|' { PT _ (TS _ 25) }
  '\\sin\\left(' { PT _ (TS _ 26) }
  '\\sin^{-1}\\left(' { PT _ (TS _ 27) }
  '\\sqrt[' { PT _ (TS _ 28) }
  '\\sqrt{' { PT _ (TS _ 29) }
  '\\tan\\left(' { PT _ (TS _ 30) }
  '\\tan^{-1}\\left(' { PT _ (TS _ 31) }
  ']' { PT _ (TS _ 32) }
  ']{' { PT _ (TS _ 33) }
  '^{' { PT _ (TS _ 34) }
  'e' { PT _ (TS _ 35) }
  '}' { PT _ (TS _ 36) }
  '}{' { PT _ (TS _ 37) }
  L_doubl  { PT _ (TD $$) }
  L_integ  { PT _ (TI $$) }
  L_Id { PT _ (T_Id $$) }

%%

Double  :: { Double }
Double   : L_doubl  { (read $1) :: Double }

Integer :: { Integer }
Integer  : L_integ  { (read $1) :: Integer }

Id :: { AbsDesmos.Id }
Id  : L_Id { AbsDesmos.Id $1 }

Exp1 :: { AbsDesmos.Exp }
Exp1 : Exp1 '=' Exp2 { AbsDesmos.Equal $1 $3 } | Exp2 { $1 }

Exp2 :: { AbsDesmos.Exp }
Exp2 : Exp2 '+' Exp3 { AbsDesmos.Plus $1 $3 }
     | Exp2 '-' Exp3 { AbsDesmos.Minus $1 $3 }
     | Exp3 { $1 }

Exp3 :: { AbsDesmos.Exp }
Exp3 : Exp3 '\\cdot' Exp4 { AbsDesmos.Times $1 $3 }
     | '-' Exp4 { AbsDesmos.Negate $2 }
     | Exp4 '!' { AbsDesmos.Fact $1 }
     | '\\frac{' Exp3 '}{' Exp4 '}' { AbsDesmos.Div $2 $4 }
     | Exp4 { $1 }

Exp4 :: { AbsDesmos.Exp }
Exp4 : Exp4 '^{' Exp5 '}' { AbsDesmos.Exp $1 $3 }
     | '\\sqrt{' Exp5 '}' { AbsDesmos.Sqrt $2 }
     | '\\sqrt[' Exp6 ']{' Exp5 '}' { AbsDesmos.NRt $2 $4 }
     | Exp5 { $1 }

Exp5 :: { AbsDesmos.Exp }
Exp5 : '\\sin\\left(' Exp6 '\\right)' { AbsDesmos.Sin $2 }
     | '\\cos\\left(' Exp6 '\\right)' { AbsDesmos.Cos $2 }
     | '\\tan\\left(' Exp6 '\\right)' { AbsDesmos.Tan $2 }
     | '\\sin^{-1}\\left(' Exp6 '\\right)' { AbsDesmos.ASin $2 }
     | '\\cos^{-1}\\left(' Exp6 '\\right)' { AbsDesmos.ACos $2 }
     | '\\tan^{-1}\\left(' Exp6 '\\right)' { AbsDesmos.ATan $2 }
     | '\\ln' Exp6 { AbsDesmos.Ln $2 }
     | '\\log' Exp6 { AbsDesmos.Log $2 }
     | Exp6 { $1 }

Exp6 :: { AbsDesmos.Exp }
Exp6 : '\\operatorname{mean}\\left(' Exp7 '\\right)' { AbsDesmos.Mean $2 }
     | '\\operatorname{stdev}\\left(' Exp7 '\\right)' { AbsDesmos.Stdev $2 }
     | '\\operatorname{stdevp}\\left(' Exp7 '\\right)' { AbsDesmos.StdevP $2 }
     | '\\operatorname{nPr}\\left(' Exp7 '\\right)' { AbsDesmos.NPR $2 }
     | '\\operatorname{nCr}\\left(' Exp7 '\\right)' { AbsDesmos.NCR $2 }
     | '\\operatorname{round}\\left(' Exp7 '\\right)' { AbsDesmos.Round $2 }
     | Exp7 '\\%\\operatorname{of}' Exp8 { AbsDesmos.Percent $1 $3 }
     | Exp7 { $1 }

Exp7 :: { AbsDesmos.Exp }
Exp7 : '\\left|' Exp8 '\\right|' { AbsDesmos.Abs $2 } | Exp8 { $1 }

Exp8 :: { AbsDesmos.Exp }
Exp8 : Exp8 ',' Exp9 { AbsDesmos.Elems $1 $3 }
     | '[' Exp8 ']' { AbsDesmos.List $2 }
     | Exp8 '[' Exp9 ']' { AbsDesmos.Index $1 $3 }
     | Exp9 { $1 }

Exp9 :: { AbsDesmos.Exp }
Exp9 : 'e' { AbsDesmos.E }
     | '\\pi' { AbsDesmos.Pi }
     | Integer { AbsDesmos.Int $1 }
     | Double { AbsDesmos.Dbl $1 }
     | Exp10 { $1 }

Exp10 :: { AbsDesmos.Exp }
Exp10 : Id { AbsDesmos.Var $1 } | Exp11 { $1 }

Exp :: { AbsDesmos.Exp }
Exp : Exp1 { $1 }

Exp11 :: { AbsDesmos.Exp }
Exp11 : '(' Exp ')' { $2 }
{

type Err = Either String

happyError :: [Token] -> Err a
happyError ts = Left $
  "syntax error at " ++ tokenPos ts ++
  case ts of
    []      -> []
    [Err _] -> " due to lexer error"
    t:_     -> " before `" ++ (prToken t) ++ "'"

myLexer :: String -> [Token]
myLexer = tokens

}

