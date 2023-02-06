{-# LANGUAGE CPP #-}
{-# LANGUAGE FlexibleInstances #-}
{-# LANGUAGE LambdaCase #-}
#if __GLASGOW_HASKELL__ <= 708
{-# LANGUAGE OverlappingInstances #-}
#endif

{-# OPTIONS_GHC -fno-warn-incomplete-patterns #-}

-- | Pretty-printer for PrintDesmos.
--   Generated by the BNF converter.

module PrintDesmos where

import Prelude
  ( ($), (.)
  , Bool(..), (==), (<)
  , Int, Integer, Double, (+), (-), (*)
  , String, (++)
  , ShowS, showChar, showString
  , all, dropWhile, elem, foldr, id, map, null, replicate, shows, span
  )
import Data.Char ( Char, isSpace )
import qualified AbsDesmos

-- | The top-level printing method.

printTree :: Print a => a -> String
printTree = render . prt 0

type Doc = [ShowS] -> [ShowS]

doc :: ShowS -> Doc
doc = (:)

render :: Doc -> String
render d = rend 0 (map ($ "") $ d []) "" where
  rend i = \case
    "["      :ts -> showChar '[' . rend i ts
    "("      :ts -> showChar '(' . rend i ts
    "{"      :ts -> showChar '{' . new (i+1) . rend (i+1) ts
    "}" : ";":ts -> new (i-1) . space "}" . showChar ';' . new (i-1) . rend (i-1) ts
    "}"      :ts -> new (i-1) . showChar '}' . new (i-1) . rend (i-1) ts
    [";"]        -> showChar ';'
    ";"      :ts -> showChar ';' . new i . rend i ts
    t  : ts@(p:_) | closingOrPunctuation p -> showString t . rend i ts
    t        :ts -> space t . rend i ts
    _            -> id
  new i     = showChar '\n' . replicateS (2*i) (showChar ' ') . dropWhile isSpace
  space t s =
    case (all isSpace t', null spc, null rest) of
      (True , _   , True ) -> []              -- remove trailing space
      (False, _   , True ) -> t'              -- remove trailing space
      (False, True, False) -> t' ++ ' ' : s   -- add space if none
      _                    -> t' ++ s
    where
      t'          = showString t []
      (spc, rest) = span isSpace s

  closingOrPunctuation :: String -> Bool
  closingOrPunctuation [c] = c `elem` closerOrPunct
  closingOrPunctuation _   = False

  closerOrPunct :: String
  closerOrPunct = ")],;"

parenth :: Doc -> Doc
parenth ss = doc (showChar '(') . ss . doc (showChar ')')

concatS :: [ShowS] -> ShowS
concatS = foldr (.) id

concatD :: [Doc] -> Doc
concatD = foldr (.) id

replicateS :: Int -> ShowS -> ShowS
replicateS n f = concatS (replicate n f)

-- | The printer class does the job.

class Print a where
  prt :: Int -> a -> Doc
  prtList :: Int -> [a] -> Doc
  prtList i = concatD . map (prt i)

instance {-# OVERLAPPABLE #-} Print a => Print [a] where
  prt = prtList

instance Print Char where
  prt     _ s = doc (showChar '\'' . mkEsc '\'' s . showChar '\'')
  prtList _ s = doc (showChar '"' . concatS (map (mkEsc '"') s) . showChar '"')

mkEsc :: Char -> Char -> ShowS
mkEsc q = \case
  s | s == q -> showChar '\\' . showChar s
  '\\' -> showString "\\\\"
  '\n' -> showString "\\n"
  '\t' -> showString "\\t"
  s -> showChar s

prPrec :: Int -> Int -> Doc -> Doc
prPrec i j = if j < i then parenth else id

instance Print Integer where
  prt _ x = doc (shows x)

instance Print Double where
  prt _ x = doc (shows x)

instance Print AbsDesmos.Id where
  prt _ (AbsDesmos.Id i) = doc $ showString i

instance Print AbsDesmos.Exp where
  prt i = \case
    AbsDesmos.Equal exp1 exp2 -> prPrec i 1 (concatD [prt 1 exp1, doc (showString "="), prt 2 exp2])
    AbsDesmos.Plus exp1 exp2 -> prPrec i 2 (concatD [prt 2 exp1, doc (showString "+"), prt 3 exp2])
    AbsDesmos.Minus exp1 exp2 -> prPrec i 2 (concatD [prt 2 exp1, doc (showString "-"), prt 3 exp2])
    AbsDesmos.Times exp1 exp2 -> prPrec i 3 (concatD [prt 3 exp1, doc (showString "\\cdot"), prt 4 exp2])
    AbsDesmos.Negate exp -> prPrec i 3 (concatD [doc (showString "-"), prt 4 exp])
    AbsDesmos.Fact exp -> prPrec i 3 (concatD [prt 4 exp, doc (showString "!")])
    AbsDesmos.Div exp1 exp2 -> prPrec i 3 (concatD [doc (showString "\\frac{"), prt 3 exp1, doc (showString "}{"), prt 4 exp2, doc (showString "}")])
    AbsDesmos.Exp exp1 exp2 -> prPrec i 4 (concatD [prt 4 exp1, doc (showString "^{"), prt 5 exp2, doc (showString "}")])
    AbsDesmos.Sqrt exp -> prPrec i 4 (concatD [doc (showString "\\sqrt{"), prt 5 exp, doc (showString "}")])
    AbsDesmos.NRt exp1 exp2 -> prPrec i 4 (concatD [doc (showString "\\sqrt["), prt 6 exp1, doc (showString "]{"), prt 5 exp2, doc (showString "}")])
    AbsDesmos.Sin exp -> prPrec i 5 (concatD [doc (showString "\\sin\\left("), prt 6 exp, doc (showString "\\right)")])
    AbsDesmos.Cos exp -> prPrec i 5 (concatD [doc (showString "\\cos\\left("), prt 6 exp, doc (showString "\\right)")])
    AbsDesmos.Tan exp -> prPrec i 5 (concatD [doc (showString "\\tan\\left("), prt 6 exp, doc (showString "\\right)")])
    AbsDesmos.ASin exp -> prPrec i 5 (concatD [doc (showString "\\sin^{-1}\\left("), prt 6 exp, doc (showString "\\right)")])
    AbsDesmos.ACos exp -> prPrec i 5 (concatD [doc (showString "\\cos^{-1}\\left("), prt 6 exp, doc (showString "\\right)")])
    AbsDesmos.ATan exp -> prPrec i 5 (concatD [doc (showString "\\tan^{-1}\\left("), prt 6 exp, doc (showString "\\right)")])
    AbsDesmos.Ln exp -> prPrec i 5 (concatD [doc (showString "\\ln"), prt 6 exp])
    AbsDesmos.Log exp -> prPrec i 5 (concatD [doc (showString "\\log"), prt 6 exp])
    AbsDesmos.Mean exp -> prPrec i 6 (concatD [doc (showString "\\operatorname{mean}\\left("), prt 7 exp, doc (showString "\\right)")])
    AbsDesmos.Stdev exp -> prPrec i 6 (concatD [doc (showString "\\operatorname{stdev}\\left("), prt 7 exp, doc (showString "\\right)")])
    AbsDesmos.StdevP exp -> prPrec i 6 (concatD [doc (showString "\\operatorname{stdevp}\\left("), prt 7 exp, doc (showString "\\right)")])
    AbsDesmos.NPR exp -> prPrec i 6 (concatD [doc (showString "\\operatorname{nPr}\\left("), prt 7 exp, doc (showString "\\right)")])
    AbsDesmos.NCR exp -> prPrec i 6 (concatD [doc (showString "\\operatorname{nCr}\\left("), prt 7 exp, doc (showString "\\right)")])
    AbsDesmos.Round exp -> prPrec i 6 (concatD [doc (showString "\\operatorname{round}\\left("), prt 7 exp, doc (showString "\\right)")])
    AbsDesmos.Percent exp1 exp2 -> prPrec i 6 (concatD [prt 7 exp1, doc (showString "\\%\\operatorname{of}"), prt 8 exp2])
    AbsDesmos.Abs exp -> prPrec i 7 (concatD [doc (showString "\\left|"), prt 8 exp, doc (showString "\\right|")])
    AbsDesmos.Elems exp1 exp2 -> prPrec i 8 (concatD [prt 8 exp1, doc (showString ","), prt 9 exp2])
    AbsDesmos.List exp -> prPrec i 8 (concatD [doc (showString "["), prt 8 exp, doc (showString "]")])
    AbsDesmos.Index exp1 exp2 -> prPrec i 8 (concatD [prt 8 exp1, doc (showString "["), prt 9 exp2, doc (showString "]")])
    AbsDesmos.E -> prPrec i 9 (concatD [doc (showString "e")])
    AbsDesmos.Pi -> prPrec i 9 (concatD [doc (showString "\\pi")])
    AbsDesmos.Int n -> prPrec i 9 (concatD [prt 0 n])
    AbsDesmos.Dbl d -> prPrec i 9 (concatD [prt 0 d])
    AbsDesmos.Var id_ -> prPrec i 10 (concatD [prt 0 id_])
