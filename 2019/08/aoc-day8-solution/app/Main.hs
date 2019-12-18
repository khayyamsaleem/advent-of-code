module Main where

import Lib

main :: IO ()
main = do
    token <- getToken
    image <- getInput "https://adventofcode.com/2019/day/8/input" token
    print $ partOne image 25 6
    putStrLn $ partTwo image 25 6