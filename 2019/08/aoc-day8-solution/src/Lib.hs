{-# LANGUAGE OverloadedStrings  #-}

module Lib
    ( getToken,
      getInput,
      partOne,
      partTwo,
    ) where

import Configuration.Dotenv
import System.Environment
import Network.HTTP.Simple
import qualified Data.Text as T
import Data.ByteString.Char8 as B hiding (length,drop,take,map,foldr,head,zip,intercalate)
import qualified Data.ByteString.Lazy.Char8 as C
import Network.HTTP.Client.Conduit (bodyReaderSource)
import Data.List
import Data.Char

data Pixel = Black | White | Transparent deriving (Show, Enum)

getToken :: IO String
getToken = do
    loadFile defaultConfig
    token <- getEnv "session"
    return token

strip :: String -> String
strip  = T.unpack . T.strip . T.pack

getInput :: String -> String -> IO String
getInput uri token = do
    let sessionCookie = "session=" ++ token
    request' <- parseRequest $ "GET " ++ uri
    let req
            = setRequestMethod "GET"
            $ setRequestHeader "Cookie" [B.pack sessionCookie]
            $ request'
    res <- httpLBS req
    return $ strip $ C.unpack $ getResponseBody res

buildLayer :: String -> Int -> Int -> [[Pixel]]
buildLayer [] _ _ = []
buildLayer _ _ 0 = []
buildLayer input x y =
    [map ( toEnum . Data.Char.digitToInt) (take x input)] ++ buildLayer (drop x input) x (y-1)

buildLayers :: String -> Int -> Int -> [[[Pixel]]]
buildLayers [] _ _ = []
buildLayers input x y =
    [(buildLayer (take (x * y) input) x y)] ++ buildLayers (drop (x*y) input) x y

flatten :: [[a]] -> [a]
flatten xs = (\z n -> foldr (\x y -> foldr z y x) n xs) (:) []

countZerosOnesAndTwos :: [[Pixel]] -> (Int,Int,Int)
countZerosOnesAndTwos layer =
    czot (flatten layer) (0,0,0) where
        czot [] ans = ans
        czot (Black:xs) (zs,os,ts) = czot xs (zs+1,os,ts)
        czot (White:xs) (zs,os,ts) = czot xs (zs,os+1,ts)
        czot (Transparent:xs) (zs,os,ts) = czot xs (zs,os,ts+1)


partOne :: String -> Int -> Int -> Int
partOne input x y =
    helper (map countZerosOnesAndTwos $ buildLayers input x y) (maxBound::Int) (maxBound::Int) where
        helper [] _ ans = ans
        helper ((zeros,ones,twos):xs) minZeros _ | zeros < minZeros = helper xs zeros (ones*twos)
        helper (x:xs) minZeros ans = helper xs minZeros ans


composeLayers :: [[[Pixel]]] -> [[Pixel]]
composeLayers [] = []
composeLayers (x:[]) = x
composeLayers (one:two:xs) =
    composeLayers ([(map (\(x,y) -> map (\(a,b) -> composePixels a b) (zip x y)) (zip one two))] ++ xs) where
    composePixels :: Pixel -> Pixel -> Pixel
    composePixels Transparent other = other
    composePixels other _ = other

renderImage :: [[Pixel]] -> String
renderImage image =
    intercalate "\n" $ map (\x -> map (\pixel -> case pixel of 
        White -> '*'
        Black -> ' '
        Transparent -> '_') x) image

partTwo :: String -> Int -> Int -> String
partTwo input x y =
    renderImage $ composeLayers $ buildLayers input x y