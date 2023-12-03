#include "lib/lib.hpp"
#include <string>

class reveal {
public:
  int red{0};
  int blue{0};
  int green{0};

  reveal(std::string reveal) {
    auto split = split_string(reveal, ",");
    for (auto pair : split) {
      auto [amount_string, color] = split_string_once(trim_copy(pair), " ");
      int amount = std::stoi(trim_copy(amount_string));
      if (color == "blue") {
        blue = amount;
      } else if (color == "red") {
        red = amount;
      } else {
        green = amount;
      }
    }
  }
};

class game {
  int id;
  std::vector<reveal> reveals;

public:
  game(std::string line) {
    auto [description, reveals] = split_string_once(line, ":");
    auto [_, id] = split_string_once(description, " ");
    this->id = std::stoi(id);
    auto reveals_split = split_string(reveals, ";");
    for (auto r : reveals_split) {
      this->reveals.push_back(reveal{r});
    }
  }

  int is_valid(int red, int green, int blue) {
    for (auto reveal : reveals) {
      if (reveal.red > red || reveal.green > green || reveal.blue > blue) {
        return false;
      }
    }
    return true;
  }

  uint64_t power() {
    uint64_t red = 0;
    uint64_t green = 0;
    uint64_t blue = 0;
    for (auto reveal : reveals) {
      red = fmax(red, reveal.red);
      green = fmax(green, reveal.green);
      blue = fmax(blue, reveal.blue);
    }
    return red * blue * green;
  }

  int get_id() { return id; }
};

int part_1(std::vector<game> *games) {
  int sum = 0;
  for (auto game : *games) {
    if (game.is_valid(12, 13, 14)) {
      sum += game.get_id();
    }
  }
  return sum;
}

uint64_t part_2(std::vector<game> *games) {
  uint64_t sum = 0;
  for (auto game : *games) {
    sum += game.power();
  }
  return sum;
}

int main() {
  auto lines = read_input_file("02");
  std::vector<game> games;
  for (std::string line : lines) {
    games.push_back(game{line});
  }

  print_part_1(part_1(&games));
  print_part_2(part_2(&games));
  return 0;
}
