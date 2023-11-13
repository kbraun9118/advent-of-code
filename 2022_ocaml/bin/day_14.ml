module CoordRange : sig
  type t

  val create : string * string -> t
  val to_seq : t -> (int * int) Seq.t
end = struct
  type t = (int * int) Seq.t

  let create (from, too) =
    let from_x, from_y = Aoc.split_once ',' from in
    let from_x, from_y = (int_of_string from_x, int_of_string from_y) in
    let to_x, to_y = Aoc.split_once ',' too in
    let to_x, to_y = (int_of_string to_x, int_of_string to_y) in
    match ((from_x, from_y), (to_x, to_y)) with
    | (from_x, from_y), (to_x, to_y) when from_x = to_x && from_y < to_y ->
        Seq.init (to_y - from_y + 1) (fun y -> (from_x, y + from_y))
    | (from_x, from_y), (to_x, to_y) when from_x = to_x ->
        Seq.init (from_y - to_y + 1) (fun y -> (from_x, y + to_y))
    | (from_x, from_y), (to_x, to_y) when from_y = to_y && from_x < to_x ->
        Seq.init (to_x - from_x + 1) (fun x -> (x + from_x, from_y))
    | (from_x, from_y), (to_x, to_y) when from_y = to_y ->
        Seq.init (from_x - to_x + 1) (fun x -> (x + to_x, from_y))
    | _ -> failwith "invalid range"

  let to_seq t = t
end

module Cave : sig
  type t

  val init : unit -> t
  val add_range : CoordRange.t -> t -> t
  val sand_count : t -> int
  val drop_sand : t -> t option
  val drop_sand2 : t -> t option
end = struct
  open Aoc

  type t = { set : Aoc.IntTupleSet.t; rock_count : int; max_y : int }

  let init () = { set = IntTupleSet.empty; rock_count = 0; max_y = 0 }

  let add_range range t =
    let set =
      Seq.fold_left
        (fun acc coord -> IntTupleSet.add coord acc)
        t.set (CoordRange.to_seq range)
    in
    {
      set;
      rock_count = Seq.length @@ IntTupleSet.to_seq set;
      max_y = IntTupleSet.fold (fun (_, y) acc -> max acc y) set 0;
    }

  let sand_count t = (IntTupleSet.to_list t.set |> List.length) - t.rock_count

  let contains_coord coord t =
    match IntTupleSet.find_opt coord t.set with Some _ -> true | None -> false

  let contains_coord_floor (x, y) t = y = t.max_y + 2 || contains_coord (x, y) t

  let drop_sand t =
    let rec drop_sand (x, y) t =
      if y + 1 > t.max_y then None
      else if not @@ contains_coord (x, y + 1) t then drop_sand (x, y + 1) t
      else if not @@ contains_coord (x - 1, y + 1) t then
        drop_sand (x - 1, y + 1) t
      else if not @@ contains_coord (x + 1, y + 1) t then
        drop_sand (x + 1, y + 1) t
      else Some (x, y)
    in
    let coord = drop_sand (500, 0) t in
    Option.map (fun coord -> { t with set = IntTupleSet.add coord t.set }) coord

  let drop_sand2 t =
    let rec drop_sand (x, y) t =
      if contains_coord_floor (500, 0) t then None
      else if not @@ contains_coord_floor (x, y + 1) t then
        drop_sand (x, y + 1) t
      else if not @@ contains_coord_floor (x - 1, y + 1) t then
        drop_sand (x - 1, y + 1) t
      else if not @@ contains_coord_floor (x + 1, y + 1) t then
        drop_sand (x + 1, y + 1) t
      else Some (x, y)
    in
    let coord = drop_sand (500, 0) t in
    Option.map (fun coord -> { t with set = IntTupleSet.add coord t.set }) coord
end

let part1 cave =
  let rec drop_until_none cave =
    let next = Cave.drop_sand cave in
    match next with Some cave -> drop_until_none cave | None -> cave
  in
  drop_until_none cave |> Cave.sand_count

let part2 cave =
  let rec drop_until_none cave =
    let next = Cave.drop_sand2 cave in
    match next with Some cave -> drop_until_none cave | None -> cave
  in
  drop_until_none cave |> Cave.sand_count

let () =
  let coords =
    Aoc.read_lines "./input/day_14/input.txt"
    |> List.map (fun line ->
           String.split_on_char ' ' line
           |> List.filter (fun nums -> nums <> "->")
           |> Aoc.window 2
           |> List.map (fun doubles ->
                  (List.hd doubles, List.tl doubles |> List.hd)))
    |> List.flatten |> List.map CoordRange.create
  in
  let cave =
    List.fold_left
      (fun cave cords -> Cave.add_range cords cave)
      (Cave.init ()) coords
  in
  let part1 = part1 cave in
  let part2 = part2 cave in
  Printf.printf "Part 1: %d\nPart 2: %d\n" part1 part2
