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

  val create : string -> int list -> t
  val run_once : t -> int
  val run_until_end : t -> int
end = struct
  type t = Lib.IntCode.t array

  let create input aplifiers =
    Array.init 5 (fun i ->
        Lib.IntCode.create ~argument:(List.nth aplifiers i) input)

  let run_once circuit =
    let run_circuit signal aplifier =
      Lib.IntCode.push_input signal aplifier
      |> Lib.IntCode.execute_until_output |> Lib.IntCode.output
    in
    Array.fold_left run_circuit 0 circuit

  let run_until_end circuit =
    let rec run_until_end previous output circuit =
      if Array.for_all Lib.IntCode.is_complete circuit then
        output
      else
        let circuit = Array.map Lib.IntCode.clear_output circuit in
        circuit.(0) <-
          Lib.IntCode.push_input previous circuit.(0)
          |> Lib.IntCode.execute_until_output;
        for i = 1 to 4 do
          circuit.(i) <-
            Lib.IntCode.push_input
              (Lib.IntCode.output circuit.(i - 1))
              circuit.(i)
            |> Lib.IntCode.execute_until_output
        done;
        run_until_end (Lib.IntCode.output circuit.(4)) previous circuit
    in
    run_until_end 0 0 circuit
end

let part_1 input =
  combinations 0 4
  |> List.map (AmpCircuit.create input)
  |> List.map AmpCircuit.run_once
  |> List.fold_left max Int.min_int

let part_2 input =
  combinations 5 9
  |> List.map (AmpCircuit.create input)
  |> List.map AmpCircuit.run_until_end
  |> List.fold_left max Int.min_int

let () =
  let input = Lib.get_input_lines "07" |> List.hd in
  Printf.printf "Part 1: %d\n" @@ part_1 input;
  Printf.printf "Part 2: %d\n" @@ part_2 input
