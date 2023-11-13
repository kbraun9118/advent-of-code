module CharSet = Set.Make (Char)

let split str =
  let half_str = String.length str / 2 in
  [ String.sub str 0 half_str; String.sub str half_str half_str ]

let window list =
  let rec window_tail acc = function
    | [] -> acc
    | x :: y :: z :: tl -> window_tail ([ x; y; z ] :: acc) tl
    | _ -> failwith "Must have at least 3 items to window"
  in
  window_tail [] list

let score char =
  let code = Char.code char in
  match char with
  | 'a' .. 'z' -> code - 96
  | 'A' .. 'Z' -> code - 38
  | _ -> failwith @@ Printf.sprintf "Invalid character: %c" char

let inter_sets = function
  | [] -> failwith "Cannot inter on empty list"
  | hd :: tl -> List.fold_left CharSet.inter hd tl

let find_duplicate list =
  let f string = String.to_seq string |> CharSet.of_seq in
  List.map f list |> inter_sets |> CharSet.choose |> score

let part1 list =
  List.map split list |> List.map find_duplicate |> List.fold_left ( + ) 0

let part2 list =
  window list |> List.map find_duplicate |> List.fold_left ( + ) 0

let () =
  let lines = Aoc.read_lines "./input/day_03/input.txt" in
  let () = Printf.printf "Part 1: %d\n" @@ part1 lines in
  Printf.printf "Part 2: %d\n" @@ part2 lines
