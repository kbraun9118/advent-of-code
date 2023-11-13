module HeightMap : sig
  type t = { height : int; width : int; map : char array array }

  val create : string list -> t
  val shortest_length_from_start : t -> int * int -> int
  val find_start : t -> int * int
  val lowest_positions : t -> (int * int) list
end = struct
  type t = { height : int; width : int; map : char array array }

  let create lines =
    let height = List.length lines in
    let width = List.hd lines |> String.length in
    let map = Array.make_matrix height width '-' in
    for y = 0 to height - 1 do
      for x = 0 to width - 1 do
        map.(y).(x) <- String.get (List.nth lines y) x
      done
    done;
    { height; width; map }

  let get ~x ~y t = t.map.(y).(x)

  let code = function
    | 'E' -> Char.code 'z'
    | 'S' -> Char.code 'a'
    | char -> Char.code char

  let increment char = Char.chr @@ (code char + 1)

  let movable_neighbor (x, y) t =
    [ (x + 1, y); (x - 1, y); (x, y + 1); (x, y - 1) ]
    |> List.filter (fun (x, y) ->
           x <> -1 && x <> t.width && y <> -1 && y <> t.height)
    |> List.filter (fun (nx, ny) ->
           Char.chr @@ code @@ get ~x:nx ~y:ny t <= increment @@ get ~x ~y t)

  let pos_of_char c t =
    Array.mapi
      (fun y row -> Array.mapi (fun x _ -> (x, y)) row |> Array.to_list)
      t.map
    |> Array.to_list |> List.flatten
    |> List.find (fun (x, y) -> t.map.(y).(x) = c)

  let shortest_length_from_start map (sx, sy) =
    let dist = Array.make_matrix map.height map.width Int.max_int in
    let prev = Array.make_matrix map.height map.width None in
    let q = Queue.create () in
    let iterate_row y row = Array.iteri (fun x _ -> Queue.add (x, y) q) row in
    Array.iteri iterate_row map.map;
    let q = Queue.to_seq q |> List.of_seq in
    dist.(sy).(sx) <- 0;
    let end_x, end_y = pos_of_char 'E' map in
    let rec find_min_connection q =
      match q with
      | [] -> ()
      | _ ->
          let ux, uy =
            let min_distance (accx, accy) (ux, uy) =
              if dist.(accy).(accx) < dist.(uy).(ux) then (accx, accy)
              else (ux, uy)
            in
            List.fold_left min_distance (List.hd q) q
          in
          if dist.(uy).(ux) = Int.max_int || (end_x = ux && end_y = uy) then ()
          else
            let q = List.filter (fun (x, y) -> x <> ux || y <> uy) q in
            let neighbors = movable_neighbor (ux, uy) map in
            let neighbors =
              List.filter
                (fun (x, y) -> List.exists (fun (nx, ny) -> x = nx && y = ny) q)
                neighbors
            in
            let handle_neighbor (vx, vy) =
              let alt = dist.(uy).(ux) + 1 in
              if alt < dist.(vy).(vx) then (
                dist.(vy).(vx) <- alt;
                prev.(vy).(vx) <- Some (vx, vy))
            in
            List.iter handle_neighbor neighbors;
            find_min_connection q
    in
    find_min_connection q;
    dist.(end_y).(end_x)

  let find_start = pos_of_char 'S'

  let lowest_positions t =
    Array.mapi
      (fun y row -> Array.mapi (fun x _ -> (x, y)) row |> Array.to_list)
      t.map
    |> Array.to_list |> List.flatten
    |> List.filter (fun (x, y) -> code t.map.(y).(x) = code 'a')
end

let part1 map =
  HeightMap.shortest_length_from_start map (HeightMap.find_start map)

let part2 map =
  HeightMap.lowest_positions map
  |> List.map @@ HeightMap.shortest_length_from_start map
  |> List.fold_left min Int.max_int

let () =
  let lines = Aoc.read_lines "./input/day_12/input.txt" in
  let map = HeightMap.create lines in
  Printf.printf "Part 1: %d\n" @@ part1 map;
  Printf.printf "Part 2: %d\n" @@ part2 map;
  ()
