let combinations min max =
  let nums = Seq.init (max - min + 1) (( + ) min) in
  let rec combinations = function
    | [ x ] -> [ [ x ] ]
    | xs ->
        List.map
          (fun x ->
            combinations (List.filter (( <> ) x) xs)
            |> List.map (fun xs -> x :: xs))
          xs
        |> List.flatten
  in
  combinations (List.of_seq nums)

module AmpCircuit : sig
  type t
end = struct
  type t = Lib.IntCode.t array

  let create input = Array.init 7 (fun _ -> Lib.IntCode.create input)
end

let () =
  let combos =
    combinations 0 4 |> List.map (fun xs -> List.map string_of_int xs)
  in
  let combos =
    List.map
      (fun xs ->
        List.fold_left (fun (n : string) (acc : string) -> n ^ acc) "" xs)
      combos
  in
  List.iter print_endline combos
