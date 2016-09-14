{-# LANGUAGE ParallelListComp #-}
module Main where

import Numeric.LinearAlgebra.Data (fromList, toList, cmap)
import Numeric.GSL.Fitting.Linear (linear_w)
import Graphics.Rendering.Chart.Easy
import Graphics.Rendering.Chart.Backend.Cairo
import Text.Printf

x = fromList [10, 100, 1000, 10000, 100000, 1000000]
y = fromList [4772, 21148, 134194, 1343814, 13389821, 148509325]
sigma = fromList [2112, 8194, 61719, 804762, 4853113, 42071732]

(intercept, slope, _, _, _, chi_sq) = linear_w x (cmap (\s -> 1 / s^2) sigma) y

errbars = liftEC $ do
    let values = [symErrPoint x' y' 0 dy
                  | x' <- toList x
                  | y' <- toList y
                  | dy <- toList sigma
                 ]
    plot_errbars_values .= values

main = toFile def "benchmark-fit.png" $ do
    setColors (map opaque [blue,red,green])
    layout_title .=
        printf "t = %.2f (ns/element) n + %.2f (ns) [chi^2 = %.2f]" slope intercept chi_sq
    layout_x_axis . laxis_generate .= autoScaledLogAxis def
    layout_y_axis . laxis_generate .= autoScaledLogAxis def
    plot errbars
    plot $ points "Benchmark data"
        (zip (toList x) (toList y))
    plot $ line "Best fit" [[
            (t, slope*t + intercept)
            | t <- map (10**) [0.5, 0.6 .. 6.5 ]
        ]]
