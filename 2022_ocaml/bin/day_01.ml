let pop list =
  match list with
  | [] -> failwith "Could not pop empty list"
  | [ x ] -> (x, [])
  | x :: xs -> (x, xs)

let rec combined lines =
  match lines with
  | [] -> [ 0 ]
  | "" :: xs -> 0 :: combined xs
  | x :: xs ->
      let hd, tail = combined xs |> pop in
      (int_of_string x |> ( + ) hd) :: tail

let part1 lines =
  let ans = List.fold_left max 0 lines in
  Printf.printf "Part 1: %d\n" ans

let part2 lines =
  let rec part2 list (x1, x2, x3) =
    match list with
    | [] -> x1 + x2 + x3
    | hd :: tl -> (
        match (x1, x2, x3) with
        | x1, x2, _ when hd > x1 -> part2 tl (hd, x1, x2)
        | x1, x2, _ when hd > x2 -> part2 tl (x1, hd, x2)
        | x1, x2, x3 when hd > x3 -> part2 tl (x1, x2, hd)
        | _ -> part2 tl (x1, x2, x3))
  in
  Printf.printf "Part 2: %d\n" @@ part2 lines (0, 0, 0)

let () =
  let lines = Aoc.read_lines "./input/day_01/input.txt" in
  let lines = combined lines in
  let () = part1 lines in
  part2 lines
