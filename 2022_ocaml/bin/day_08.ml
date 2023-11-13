let next_coord (x, y) = function
  | Aoc.Up -> (x, y - 1)
  | Down -> (x, y + 1)
  | Left -> (x - 1, y)
  | Right -> (x + 1, y)

let get_visible trees (x, y) =
  let rec get_visible_in_dir trees prev max_in direction =
    let new_coord = next_coord prev direction in
    match new_coord with
    | -1, _ | _, -1 -> []
    | x, _ when x >= Array.length trees -> []
    | _, y when y >= Array.length trees.(0) -> []
    | nx, ny ->
        (if trees.(nx).(ny) <= max_in then [] else [ (nx, ny) ])
        @ get_visible_in_dir trees (nx, ny)
            (max max_in trees.(nx).(ny))
            direction
  in
  List.map
    (get_visible_in_dir trees (x, y) trees.(x).(y))
    [ Left; Right; Up; Down ]

let get_visible_from_tree trees (x, y) =
  let rec get_visible_in_dir trees prev (x, y) direction =
    let new_coord = next_coord prev direction in
    match new_coord with
    | -1, _ | _, -1 -> []
    | x, _ when x >= Array.length trees -> []
    | _, y when y >= Array.length trees.(0) -> []
    | nx, ny when trees.(nx).(ny) >= trees.(x).(y) -> [ (nx, ny) ]
    | nx, ny -> (nx, ny) :: get_visible_in_dir trees (nx, ny) (x, y) direction
  in
  List.map (get_visible_in_dir trees (x, y) (x, y)) [ Left; Right; Up; Down ]

let part_1 trees =
  let perimeter_filter coord =
    match coord with
    | 0, _ | _, 0 -> true
    | x, _ when x + 1 = Array.length trees -> true
    | _, y when y + 1 = Array.length trees.(0) -> true
    | _ -> false
  in
  let perimeter =
    Seq.init (Array.length trees) (fun x ->
        Seq.init (Array.length trees.(x)) (fun y -> (x, y)))
    |> Seq.flat_map (fun x -> x)
    |> List.of_seq
    |> List.filter perimeter_filter
  in
  List.map
    (fun (x, y) -> (x, y) :: List.flatten (get_visible trees (x, y)))
    perimeter
  |> List.flatten |> Aoc.IntTupleSet.of_list |> Aoc.IntTupleSet.to_list
  |> List.length

let part_2 trees =
  Seq.init (Array.length trees) (fun x ->
      Seq.init (Array.length trees.(x)) (fun y -> (x, y)))
  |> Seq.flat_map (fun x -> x)
  |> List.of_seq
  |> List.map (fun (x, y) -> get_visible_from_tree trees (x, y))
  |> List.map (fun l -> List.map (fun l -> List.length l) l)
  |> List.map (List.fold_left ( * ) 1)
  |> List.fold_left max 1

let parse_trees trees =
  let matrix =
    Array.make_matrix (String.length (List.nth trees 0)) (List.length trees) 0
  in
  let parse_row x row =
    String.to_seq row
    |> Seq.iteri (fun y c -> matrix.(x).(y) <- int_of_string @@ String.make 1 c)
  in
  let () = List.iteri parse_row trees in
  matrix

let () =
  let trees = Aoc.read_lines "./input/day_08/input.txt" |> parse_trees in
  Printf.printf "Part 1: %d\nPart 2: %d\n" (part_1 trees) (part_2 trees)
