module Signal : sig
  type t = SigList of t list | Num of int

  val create : string -> t
  val compare : t -> t -> int
end = struct
  type t = SigList of t list | Num of int
  type token = LeftBracket | RightBracket | Value of int

  let create_tokens str =
    let str = String.to_seq str |> List.of_seq in
    let rec create_tokens input current_num =
      match input with
      | '[' :: tl -> LeftBracket :: create_tokens tl current_num
      | ']' :: tl ->
          if List.length current_num = 0 then
            RightBracket :: create_tokens tl current_num
          else
            Value
              (List.rev current_num |> List.to_seq |> String.of_seq
             |> int_of_string)
            :: RightBracket :: create_tokens tl []
      | ',' :: tl ->
          if List.length current_num = 0 then create_tokens tl current_num
          else
            Value
              (List.rev current_num |> List.to_seq |> String.of_seq
             |> int_of_string)
            :: create_tokens tl []
      | hd :: tl -> create_tokens tl (hd :: current_num)
      | [] -> []
    in
    create_tokens str []

  let create str =
    let tokens = create_tokens str in
    let rec create current_list = function
      | Value x :: tl -> create (Num x :: current_list) tl
      | RightBracket :: tl -> (SigList (List.rev current_list), tl)
      | LeftBracket :: tl ->
          let inner, tl = create [] tl in
          create (inner :: current_list) tl
      | [] -> (List.hd current_list, [])
    in
    fst (create [] tokens)

  let compare left right =
    let rec compare_rec left right =
      match (left, right) with
      | Num left, Num right -> compare left right
      | Num left, SigList right ->
          compare_rec (SigList [ Num left ]) (SigList right)
      | SigList left, Num right ->
          compare_rec (SigList left) (SigList [ Num right ])
      | SigList [], SigList [] -> 0
      | SigList (_ :: _), SigList [] -> 1
      | SigList [], SigList (_ :: _) -> -1
      | SigList (left :: lefts), SigList (right :: rights) -> (
          match compare_rec left right with
          | 0 -> compare_rec (SigList lefts) (SigList rights)
          | x -> x)
    in
    compare_rec left right
end

let part1 lines =
  Aoc.chunk 3 lines
  |> List.map (fun line -> (List.hd line, List.hd @@ List.tl line))
  |> List.map (fun (left, right) -> (Signal.create left, Signal.create right))
  |> List.mapi (fun i (left, right) -> (i + 1, Signal.compare left right))
  |> List.filter (fun (_, x) -> x < 0)
  |> List.map fst |> List.fold_left ( + ) 0

let part2 lines =
  let lines =
    List.filter (fun line -> line <> "") lines |> List.map Signal.create
  in
  let lines = Signal.create "[[2]]" :: Signal.create "[[6]]" :: lines in
  List.sort Signal.compare lines
  |> List.mapi (fun i x -> (i + 1, x))
  |> List.filter (fun (_, signal) ->
         Signal.compare (Signal.create "[[2]]") signal = 0
         || Signal.compare (Signal.create "[[6]]") signal = 0)
  |> List.map (fun (i, _) -> i)
  |> List.fold_left ( * ) 1

let () =
  let lines = Aoc.read_lines "./input/day_13/input.txt" in
  let () = Printf.printf "Part 1: %d\n" @@ part1 lines in
  let () = Printf.printf "Part 2: %d\n" @@ part2 lines in
  ()
