let part1 l r =
  match r with
  | "X" -> ( 1 + match l with "A" -> 3 | "B" -> 0 | _ -> 6)
  | "Y" -> ( 2 + match l with "A" -> 6 | "B" -> 3 | _ -> 0)
  | _ -> ( 3 + match l with "A" -> 0 | "B" -> 6 | _ -> 3)

let part2 l r =
  match r with
  | "X" -> ( match l with "A" -> 3 | "B" -> 1 | _ -> 2)
  | "Y" -> ( 3 + match l with "A" -> 1 | "B" -> 2 | _ -> 3)
  | _ -> ( 6 + match l with "A" -> 2 | "B" -> 3 | _ -> 1)

let determine_score score_f lines =
  let f line =
    match String.split_on_char ' ' line with
    | [ x; y ] -> score_f x y
    | _ -> failwith "invalid arguments"
  in
  List.map f lines |> List.fold_left ( + ) 0

let () =
  let lines = Aoc.read_lines "./input/day_02/input.txt" in
  let () = Printf.printf "Part 1: %d\n" @@ determine_score part1 lines in
  Printf.printf "Part2: %d\n" @@ determine_score part2 lines
