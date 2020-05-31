import Numeric.Natural (Natural)

data Rectangle = Rectangle
  { width :: Natural
  , height :: Natural
  } deriving Show

area :: Rectangle -> Natural
area rectangle = width rectangle * height rectangle

canHold :: Rectangle -> Rectangle -> Bool
canHold rectangle1 rectangle2 =
  width rectangle1 > width rectangle2 && height rectangle1 > height rectangle2

square :: Natural -> Rectangle
square size = Rectangle
  { width = size
  , height = size
  }

main :: IO ()
main = do
  let rect1 = Rectangle
        { width = 30
        , height = 50
        }

  let rect2 = square 20

  let rect3 = Rectangle
        { width = 60
        , height = 45
        }

  putStrLn ("Can rect1 hold rect2? " <> show (rect1 `canHold` rect2))
  putStrLn ("Can rect1 hold rect3? " <> show (rect1 `canHold` rect3))
