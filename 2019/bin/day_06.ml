module OrbitalMap : sig
  type t

  (* val find_body : string -> t -> t *)

  (* val add_to_map : string -> t -> unit *)
  val create : string list -> t
  val to_list : t -> t list
  val orbit_path : t -> t list
  val transfer_length : string -> string -> t -> int
end = struct
  module Stringtbl = Hashtbl.Make (String)
  module Stringset = Set.Make (String)

  module Orbit = struct
    type t = { id : string; previous : t option; satelites : t Stringtbl.t }

    let create id = { id; previous = None; satelites = Stringtbl.create 0 }

    let add_to_orbit body satelite =
      let satelite = { satelite with previous = Some body } in
      Stringtbl.replace body.satelites satelite.id satelite

    let rec find id orbit =
      match Stringtbl.find_opt orbit.satelites id with
      | None ->
          Stringtbl.to_seq_values orbit.satelites
          |> Seq.filter_map (find id)
          |> Seq.uncons |> Option.map fst
      | Some body -> Some body

    let id orbit = orbit.id
  end

  type t = Orbit.t

  let find_body body map =
    let open Orbit in
    if map.id = body then Some map else find body map

  let add_to_map body satelite map =
    let body = find_body body map in
    Option.map
      (fun body -> Orbit.add_to_orbit body (Orbit.create satelite))
      body

  let create lines =
    let lines = List.map (Lib.split_once ')') lines in
    let root, satelite = List.hd lines in
    let map = Orbit.create root in
    add_to_map root satelite map |> Option.get;
    let rec create failed lines =
      match lines with
      | (body, satelite) :: tl -> (
          Printf.printf "Adding: %s\n" satelite;
          match add_to_map body satelite map with
          | Some () -> create failed tl
          | None -> create ((body, satelite) :: failed) tl)
      | [] -> (
          Printf.printf "Failed Length: %d\n" (List.length failed);
          match failed with [] -> () | failed -> create [] failed)
    in
    create [] lines;
    map

  let previous map =
    let open Orbit in
    map.previous

  let rec to_list map =
    let open Orbit in
    map
    :: (Stringtbl.to_seq_values map.satelites
       |> List.of_seq |> List.map to_list |> List.flatten)

  let rec orbit_path map =
    match previous map with
    | Some previous -> previous :: orbit_path previous
    | None -> []

  let id = Orbit.id

  let transfer_length from too map =
    let from =
      find_body from map |> Option.get |> orbit_path |> List.map id
      |> Stringset.of_list
    in
    let too =
      find_body too map |> Option.get |> orbit_path |> List.map id
      |> Stringset.of_list
    in
    (Stringset.diff from too |> Stringset.to_list)
    @ (Stringset.diff too from |> Stringset.to_list)
    |> List.length
end

let () =
  let lines = Lib.get_input_lines "06" in
  let map = OrbitalMap.create lines in
  OrbitalMap.to_list map
  |> List.map OrbitalMap.orbit_path
  |> List.map List.length |> List.fold_left ( + ) 0
  |> Printf.printf "Part 1: %d\n";
  Printf.printf "Part 2: %d\n" (OrbitalMap.transfer_length "YOU" "SAN" map);
  ()
