module OrbitalMap : sig
  type t

  val equal : t -> t -> bool
  val hash : t -> int
end = struct
  type t = { ident : string; orbits : t Option.t ; orbiting: int OribitMapTbl.t}

  let equal left right = String.equal left.ident right.ident
  let hash map = Hashtbl.hash map.ident
end

module OribitMapTbl = Hashtbl.Make (OrbitalMap)

let () =
  let lines = Lib.get_test_lines "06" in
  List.iter print_endline lines
