let run_int_code noun verb input =
  let open Lib in
  IntCode.create input
  |> IntCode.replace_value 1 noun
  |> IntCode.replace_value 2 verb
  |> IntCode.execute |> IntCode.return_code

let part_1 = run_int_code 12 2

let part_2 input =
  let noun, verb =
    Seq.ints 1
    |> Seq.map (fun i -> Seq.init 99 (( + ) 1) |> Seq.map (fun y -> (i, y)))
    |> Seq.find_map
       @@ Seq.find (fun (noun, verb) -> run_int_code noun verb input = 19690720)
    |> Option.get
  in
  (100 * noun) + verb

let () =
  let input = Lib.get_input_lines "02" |> List.hd in
  let () = Printf.printf "Part 1: %d\n" @@ part_1 input in
  let () = Printf.printf "Part 2: %d\n" @@ part_2 input in
  ()
