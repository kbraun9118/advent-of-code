#include "lib/lib.hpp"
#include <cctype>
#include <set>

class PartNumber {
  int number;
  std::set<std::tuple<int, int>> adjacents;

public:
  PartNumber(int number, std::vector<std::tuple<int, int>> adjacents)
      : number(number), adjacents({begin(adjacents), end(adjacents)}) {}

  int get_number() { return number; }

  std::vector<std::tuple<int, int>>
  get_adacents_gears(std::vector<std::string> *lines) {
    std::vector<std::tuple<int, int>> adjacent_gears;
    for (auto [x, y] : adjacents) {
      if ((*lines)[y][x] == '*') {
        adjacent_gears.push_back({x, y});
      }
    }
    return adjacent_gears;
  }
};

std::vector<std::tuple<int, int>> get_neighbors(std::vector<std::string> *lines, int x, int y) {
  std::vector<std::tuple<int, int>> neighbors;
  for (int i = -1; i < 2; i++) {
    for (int j = -1; j < 2; j++) {
      int nx = x + i;
      int ny = y + j;
      if ((i != 0 || j != 0) && nx >= 0 && nx < (*lines)[ny].length() &&
          ny >= 0 && ny < lines->size()) {
        neighbors.push_back({nx,ny});
      }
    }
  }
  return neighbors;
}

std::vector<PartNumber> find_part_numbers(std::vector<std::string> *lines) {
  std::string current = "";
  std::vector<PartNumber> output;
  std::vector<std::tuple<int, int>> adjacents;
  for (int y = 0; y < lines->size(); y++) {
    for (int x = 0; x < (*lines)[y].size(); x++) {
      char ch = (*lines)[y][x];
      if (isdigit(ch)) {
        current += ch;

        for (auto [nx, ny] : get_neighbors(lines, x, y)) {
          char neighbor = (*lines)[ny][nx];
          if (!isdigit(neighbor) && neighbor != '.') {
            adjacents.push_back({nx, ny});
          }
        }
      } else {
        if (adjacents.size() > 0) {
          output.push_back(PartNumber{std::stoi(current), adjacents});
        }
        adjacents = std::vector<std::tuple<int, int>>{};
        current = "";
      }
    }
    if (adjacents.size() > 0) {
      output.push_back(PartNumber{std::stoi(current), adjacents});
    }
    adjacents = std::vector<std::tuple<int, int>>{};
    current = "";
  }
  return output;
}

int part_1(std::vector<PartNumber> *part_numbers) {
  int output = 0;
  for (auto part_number : *part_numbers) {
    output += part_number.get_number();
  }
  return output;
}

struct HashTuple {

  template <class T1, class T2>

  size_t operator()(const std::tuple<T1, T2> &x) const {
    return get<0>(x) ^ get<1>(x);
  }
};

int part_2(std::vector<PartNumber> *part_numbers,
           std::vector<std::string> *lines) {
  std::unordered_map<std::tuple<int, int>, std::vector<PartNumber>, HashTuple>
      gears;
  int output{0};
  for (auto part_number : *part_numbers) {
    for (auto gear : part_number.get_adacents_gears(lines)) {
      if (gears.contains(gear)) {
        gears[gear].push_back(part_number);
      } else {
        gears[gear] = std::vector<PartNumber>{part_number};
      }
    }
  }

  for (auto [_, part_numbers] : gears) {
    if (part_numbers.size() == 2) {

      int gear_ratio = 1;
      for (auto part_number : part_numbers) {
        gear_ratio *= part_number.get_number();
      }
      output += gear_ratio;
    }
  }

  return output;
}

int main() {
  auto lines = read_input_file("03");
  auto part_numbers = find_part_numbers(&lines);

  print_part_1(part_1(&part_numbers));
  print_part_2(part_2(&part_numbers, &lines));
}
