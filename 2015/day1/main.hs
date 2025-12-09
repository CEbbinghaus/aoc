import System.Environment

main :: IO ()
main = do
    args <- getArgs
    -- grab the first command-line argument
    file <- case args of
        (x:_) -> return x
        []    -> error "No command-line arguments provided."

    -- read the file contents
    contents <- readFile file

    putStrLn $ "Solution 1: " ++ show (solution1 contents)
    putStrLn $ "Solution 2: " ++ show (solution2 contents)




solution1 :: String -> Int
-- map every character of the string to a number
solution1 str = sum $ map charToNumber str
    where
        charToNumber c
            | c == '(' = 1
            | c == ')' = -1
            | otherwise = 0

    
solution2 :: String -> Int
-- find first position where cumulative sum reaches -1
solution2 str = findPosition (zip [1..] (scanl (+) 0 (map charToNumber str))) - 1
    where
        findPosition [] = 0
        findPosition ((pos, sum):xs)
            | sum == -1 = pos
            | otherwise = findPosition xs

        charToNumber c
            | c == '(' = 1
            | c == ')' = -1
            | otherwise = 0

    
