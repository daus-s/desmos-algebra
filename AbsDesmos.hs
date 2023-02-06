-- Haskell data types for the abstract syntax.
-- Generated by the BNF converter.

{-# LANGUAGE GeneralizedNewtypeDeriving #-}

-- | The abstract syntax of language desmos.

module AbsDesmos where

import Prelude (Double, Integer, String)
import qualified Prelude as C (Eq, Ord, Show, Read)
import qualified Data.String

data Exp
    = Equal Exp Exp
    | Plus Exp Exp
    | Minus Exp Exp
    | Times Exp Exp
    | Negate Exp
    | Fact Exp
    | Div Exp Exp
    | Exp Exp Exp
    | Sqrt Exp
    | NRt Exp Exp
    | Sin Exp
    | Cos Exp
    | Tan Exp
    | ASin Exp
    | ACos Exp
    | ATan Exp
    | Ln Exp
    | Log Exp
    | Mean Exp
    | Stdev Exp
    | StdevP Exp
    | NPR Exp
    | NCR Exp
    | Round Exp
    | Percent Exp Exp
    | Abs Exp
    | Elems Exp Exp
    | List Exp
    | Index Exp Exp
    | E
    | Pi
    | Int Integer
    | Dbl Double
    | Var Id
  deriving (C.Eq, C.Ord, C.Show, C.Read)

newtype Id = Id String
  deriving (C.Eq, C.Ord, C.Show, C.Read, Data.String.IsString)
