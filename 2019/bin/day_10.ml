module IntPairs = struct
  type t = int * int

  let compare (x0, y0) (x1, y1) =
    match Stdlib.compare x0 x1 with 0 -> Stdlib.compare y0 y1 | c -> c
end

module PairsSet = Set.Make (IntPairs)

module Grid : sig
  type t = char list list

  val init : string list -> t

  (* val get : int -> int -> t -> char *)
  val asteroids : t -> PairsSet.t
end = struct
  type t = char list list

  let init lines =
    List.map (fun line -> String.to_seq line |> List.of_seq) lines

  let get x y grid = List.nth (List.nth grid y) x

  let asteroids grid =
    List.mapi
      (fun y items ->
        List.mapi (fun x c -> (x, c)) items
        |> List.filter_map (fun (x, c) ->
               if c == '#' then Some (x, y) else None))
      grid
    |> List.concat |> PairsSet.of_list
end

let ( =. ) a b = abs_float (a -. b) < Float.epsilon

(* Going to compare slopes then see if distances eq *)
let slopes_equal (source_x, source_y) (dest_x, dest_y) (mid_x, mid_y) =
  if source_y == dest_y && source_y == mid_y then true
  else
    let slopeAB =
      Float.div
        (float_of_int (dest_y - source_y))
        (float_of_int (dest_x - source_x))
    in
    let slopeAC =
      Float.div
        (float_of_int (mid_y - source_y))
        (float_of_int (mid_x - source_x))
    in
    slopeAB =. slopeAC

let distances_sum source dest mid =
  let distance (source_x, source_y) (dest_x, dest_y) =
    sqrt
      ((float_of_int (source_x - dest_x) ** 2.0)
      +. (float_of_int (source_y - dest_y) ** 2.0))
  in
  let ab = distance source dest in
  let ac = distance source mid in
  let bc = distance mid dest in
  ab =. ac +. bc

let intercepts source dest mid =
  distances_sum source dest mid && slopes_equal source dest mid

let () =
  let input = Lib.get_test_lines "10" in
  let grid = Grid.init input in
  let asteroids = Grid.asteroids grid in
  PairsSet.iter
    (fun (x, y) -> Printf.printf "Asteroid at (%d, %d)\n" x y)
    asteroids
