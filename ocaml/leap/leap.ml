let leap_year = function
    | x when x mod 4 <> 0 -> false
    | x when x mod 400 = 0 -> true
    | x when x mod 100 = 0 -> false
    | _ -> true
;;
