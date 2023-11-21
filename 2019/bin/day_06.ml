module OrbitalMap : sig
  type t

  (* val find_body : string -> t -> t *)

  (* val add_to_map : string -> t -> unit *)
  val create : string list -> t
  val previous : t -> t option
  val to_list : t -> t list
  val id : t -> string
end = struct
  module Stringtbl = Hashtbl.Make (String)

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
  end

  type t = Orbit.t

  let find_body id map =
    let open Orbit in
    if map.id = id then map
    else
      find id map |> Option.to_result ~none:()
      |> Result.map_error (fun _ -> failwith ("No orbit found with id: " ^ id))
      |> Result.get_ok

  let add_to_map body satelite map =
    let body = find_body body map in
    Orbit.add_to_orbit body (Orbit.create satelite)

  let create lines =
    let lines = List.map (Lib.split_once ')') lines in
    let root, satelite = List.hd lines in
    let map = Orbit.create root in
    add_to_map root satelite map;
    (* |> Seq.iter (fun _x -> ()); *)
    let add_satelites (body, satelite) = add_to_map body satelite map in
    List.iter add_satelites (List.tl lines);
    map

  let previous map =
    let open Orbit in
    map.previous

  let rec to_list map =
    let open Orbit in
    map
    :: (Stringtbl.to_seq_values map.satelites
       |> List.of_seq |> List.map to_list |> List.flatten)

  let id map =
    let open Orbit in
    map.id
end

let () =
  let lines = Lib.get_test_lines "06" in
  let map = OrbitalMap.create lines in
  let _previous = OrbitalMap.previous map in
  OrbitalMap.to_list map |> List.map OrbitalMap.id |> List.iter print_endline;
  ()
