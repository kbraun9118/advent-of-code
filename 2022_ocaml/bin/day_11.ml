module Monkey : sig
  type t = {
    items : int Queue.t;
    operation : int -> int;
    test : int;
    if_true : int;
    if_false : int;
    mutable inspections : int;
  }

  val from_strings : string List.t -> t
  val push_item : int -> t -> unit
  val pop_item : t -> int
  val is_empty : t -> bool
  val operation : t -> int -> int
  val test : t -> int
  val inspections : t -> int
  val if_true : t -> int
  val if_false : t -> int
end = struct
  type t = {
    items : int Queue.t;
    operation : int -> int;
    test : int;
    if_true : int;
    if_false : int;
    mutable inspections : int;
  }

  let create_items input =
    let _, list = Aoc.split_once ':' input in
    let queue = Queue.create () in
    let () =
      String.split_on_char ',' list
      |> List.map String.trim |> List.map int_of_string
      |> List.iter (fun x -> Queue.add x queue)
    in
    queue

  let create_operation input =
    let _, operation = Aoc.split_once '=' input in
    let parse_num num =
      if Aoc.contains_substring num "old" then None
      else
        let z = String.trim num |> int_of_string in
        Some z
    in
    let op = if String.contains operation '+' then '+' else '*' in
    let left, right = Aoc.split_once op operation in
    let op = if Char.equal '+' op then ( + ) else ( * ) in
    let left = parse_num left in
    let right = parse_num right in
    match (left, right) with
    | Some x, Some y -> fun _ -> op x y
    | Some x, None -> fun y -> op x y
    | None, Some y -> fun x -> op x y
    | _ -> fun x -> op x x

  let create_test input =
    let _, num = Aoc.split_once 'y' input in
    String.trim num |> int_of_string

  let parse_true_false input =
    let _, num = Aoc.split_once 'y' input in
    String.trim num |> int_of_string

  let from_strings = function
    | _ :: starting :: operation :: test :: test_true :: test_false :: _ ->
        {
          items = create_items starting;
          operation = create_operation operation;
          test = create_test test;
          if_true = parse_true_false test_true;
          if_false = parse_true_false test_false;
          inspections = 0;
        }
    | _ -> failwith "Invalid amount of linse"

  let push_item item monkey = Queue.push item monkey.items

  let pop_item monkey =
    monkey.inspections <- monkey.inspections + 1;
    Queue.pop monkey.items

  let is_empty monkey = Queue.is_empty monkey.items
  let inspections monkey = monkey.inspections
  let operation monkey = monkey.operation
  let test monkey = monkey.test
  let if_true monkey = monkey.if_true
  let if_false monkey = monkey.if_false
end

let two_largest list =
  let rec two_largest (a, b) = function
    | [] -> (a, b)
    | hd :: tl -> (
        match (a, b) with
        | a, _ when hd > a -> two_largest (hd, a) tl
        | a, _ when hd > b -> two_largest (a, hd) tl
        | a, b -> two_largest (a, b) tl)
  in
  two_largest (0, 0) list

let handle_monkeys monkeys duration worry_fun =
  let handle_monkey monkey =
    while not @@ Monkey.is_empty monkey do
      let value = Monkey.pop_item monkey in
      let value = Monkey.operation monkey value in
      let value = worry_fun value in
      if value mod Monkey.test monkey = 0 then
        List.nth monkeys (Monkey.if_true monkey) |> Monkey.push_item value
      else List.nth monkeys (Monkey.if_false monkey) |> Monkey.push_item value
    done
  in
  for _ = 1 to duration do
    List.iter handle_monkey monkeys
  done;
  let monkeys = List.map Monkey.inspections monkeys in
  let x, y = two_largest monkeys in
  x * y

let part_1 monkeys = handle_monkeys monkeys 20 (fun v -> v / 3)

let part_2 monkeys =
  let lcm = List.map Monkey.test monkeys |> List.fold_left ( * ) 1 in
  handle_monkeys monkeys 10_000 (fun v -> v mod lcm)

let () =
  let lines = Aoc.read_lines "./input/day_11/input.txt" |> Aoc.chunk 7 in
  let monkeys = List.map Monkey.from_strings lines in
  let () = Printf.printf "Part 1: %d\n" @@ part_1 monkeys in
  let monkeys = List.map Monkey.from_strings lines in
  let () = Printf.printf "Part 2: %d\n" @@ part_2 monkeys in
  ()
