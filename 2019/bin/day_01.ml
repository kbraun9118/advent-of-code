let calculate_fuel fuel = (fuel / 3) - 2
let part_1 lines = List.map calculate_fuel lines |> List.fold_left ( + ) 0

let part_2 lines =
  let rec calculate_fuel_additional fuel =
    let fuel = calculate_fuel fuel in
    match fuel with
    | x when x <= 0 -> 0
    | x -> x + calculate_fuel_additional fuel
  in
  List.map calculate_fuel_additional lines |> List.fold_left ( + ) 0

let () =
  let lines = Lib.get_input_lines "01" |> List.map int_of_string in
  let () = Printf.printf "Part 1: %d\n" @@ part_1 lines in
  let () = Printf.printf "Part 2: %d\n" @@ part_2 lines in
  ()
