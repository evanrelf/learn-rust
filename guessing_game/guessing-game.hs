#!/usr/bin/env nix-shell
#!nix-shell --pure -i runghc --packages "haskellPackages.ghcWithPackages (hp: with hp; [ random ])"

import System.Random (randomRIO)
import Text.Read (readMaybe)

game :: Int -> IO ()
game secretNumber = do
  putStrLn "Please input your guess."
  rawGuess <- getLine
  case readMaybe rawGuess of
    Nothing -> game secretNumber
    Just guess -> do
      putStrLn $ "You guessed: " <> show guess
      case guess `compare` secretNumber of
        LT -> putStrLn "Too small!" >> game secretNumber
        GT -> putStrLn "Too big!" >> game secretNumber
        EQ -> putStrLn "You win!"

main :: IO ()
main = do
  putStrLn "Guess the number!"
  secretNumber <- randomRIO (1, 100)
  game secretNumber
