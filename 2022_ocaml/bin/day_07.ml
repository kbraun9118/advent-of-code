type cmd = ChangeDir of string | ListDir
type dir = Directory of (string, dir) Hashtbl.t * dir Option.t | File of int
type line = Output of string | Command of cmd

let cmd_of_string input =
  if String.starts_with ~prefix:"$ ls" input then ListDir
  else ChangeDir (String.sub input 5 (String.length input - 5))

let line_of_string input =
  if String.starts_with ~prefix:"$" input then Command (cmd_of_string input)
  else Output input

let is_dir = function Directory _ -> true | _ -> false

let create_fs lines =
  let root = Directory (Hashtbl.create 0, None) in
  let rec create_fs current = function
    | [] -> ()
    | hd :: tl -> (
        match hd with
        | Output out ->
            let () =
              match current with
              | Directory (current_tbl, _) ->
                  if String.starts_with ~prefix:"dir" out then
                    let name = String.sub out 4 (String.length out - 4) in
                    let newDir = Directory (Hashtbl.create 0, Some current) in
                    Hashtbl.add current_tbl name newDir
                  else
                    let split = String.split_on_char ' ' out in
                    let size = List.nth split 0 |> int_of_string in
                    let name = List.nth split 1 in
                    Hashtbl.add current_tbl name (File size)
              | _ -> ()
            in
            create_fs current tl
        | Command cmd -> (
            match cmd with
            | ChangeDir dir -> (
                match current with
                | Directory (tbl, prev) ->
                    if String.equal dir ".." then create_fs (Option.get prev) tl
                    else
                      let current = Hashtbl.find tbl dir in
                      create_fs current tl
                | _ -> failwith "Cannot cd to file")
            | _ -> create_fs current tl))
  in
  let () = create_fs root @@ List.tl lines in
  root

let rec fs_size = function
  | Directory (tbl, _) -> Hashtbl.fold (fun _ v acc -> acc + fs_size v) tbl 0
  | File size -> size

let part_1 dir =
  let rec part_1 = function
    | Directory (tbl, _) ->
        let dirs =
          Hashtbl.to_seq_values tbl |> List.of_seq |> List.filter is_dir
          |> List.map part_1 |> List.flatten
        in
        let current_size =
          Hashtbl.to_seq_values tbl |> Seq.filter is_dir |> Seq.map fs_size
          |> List.of_seq
        in
        dirs @ current_size |> List.filter (fun s -> s <= 100_000)
    | _ -> []
  in
  let ans = part_1 dir in
  List.fold_left ( + ) 0 ans

let part_2 dir =
  let rec part_2 = function
    | Directory (tbl, _) ->
        let dirs =
          Hashtbl.to_seq_values tbl |> List.of_seq |> List.filter is_dir
          |> List.map part_2 |> List.flatten
        in
        let current_size =
          Hashtbl.to_seq_values tbl |> Seq.filter is_dir |> Seq.map fs_size
          |> List.of_seq
        in
        dirs @ current_size
    | _ -> []
  in
  let size_needed = 70_000_000 - 30_000_000 in
  let fs_size = fs_size dir in
  let sorted = part_2 dir |> List.sort compare in
  List.to_seq sorted
  |> Seq.find (fun i -> fs_size - i <= size_needed)
  |> Option.get

let () =
  let lines =
    Aoc.read_lines "./input/day_07/input.txt" |> List.map line_of_string
  in
  let fs = create_fs lines in
  let () = Printf.printf "Part 1: %d\n" @@ part_1 fs in
  let () = Printf.printf "Part 2: %d\n" @@ part_2 fs in
  ()
