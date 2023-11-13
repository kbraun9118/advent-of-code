module CharSet = Set.Make (Char)

let find_start length sequence =
  let sequences = Aoc.window length sequence in
  let rec find_start i length sequence =
    match sequence with
    | [] -> failwith "could not find start"
    | hd :: tl ->
        if CharSet.of_list hd |> CharSet.to_list |> List.length = List.length hd
        then i + length
        else find_start (i + 1) length tl
  in
  find_start 0 length sequences

let explode s = List.init (String.length s) (String.get s)

let () =
  let lines = List.nth (Aoc.read_lines "./input/day_06/input.txt") 0 in
  let lines = explode lines in
  let part1 = find_start 4 lines in
  let part2 = find_start 14 lines in
  Printf.printf "Part 1: %d\nPart 2: %d" part1 part2
