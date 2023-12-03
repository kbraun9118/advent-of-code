#include "lib/lib.hpp"
#include <cctype>
#include <cstdlib>
#include <string>
#include <vector>

using namespace std;

int part_1(vector<string> input) {
  vector<int> ints;

  for (string item : input) {
    string int_string;
    for (char c : item) {
      if (isdigit(c)) {
        int_string += c;
      }
    }
    string calibration_value;
    calibration_value += int_string.at(0);
    calibration_value += int_string.at(int_string.length() - 1);

    ints.push_back(atoi(calibration_value.c_str()));
  }

  int sum{0};
  for (int item : ints) {
    sum += item;
  }
  return sum;
}

int get_start_int(string item) {
  while (!item.empty()) {
    if (isdigit(item[0])) {
      return item[0] - '0';
    } else if (item.starts_with("one")) {
      return 1;
    } else if (item.starts_with("two")) {
      return 2;
    } else if (item.starts_with("three")) {
      return 3;
    } else if (item.starts_with("four")) {
      return 4;
    } else if (item.starts_with("five")) {
      return 5;
    } else if (item.starts_with("six")) {
      return 6;
    } else if (item.starts_with("seven")) {
      return 7;
    } else if (item.starts_with("eight")) {
      return 8;
    } else if (item.starts_with("nine")) {
      return 9;
    }
    item = item.substr(1, item.length() - 1);
  }
  return -1;
}

int get_end_int(string item) {
  while (!item.empty()) {
    int end = item.length() - 1;
    if (isdigit(item[end])) {
      return item[end] - '0';
    } else if (item.ends_with("one")) {
      return 1;
    } else if (item.ends_with("two")) {
      return 2;
    } else if (item.ends_with("three")) {
      return 3;
    } else if (item.ends_with("four")) {
      return 4;
    } else if (item.ends_with("five")) {
      return 5;
    } else if (item.ends_with("six")) {
      return 6;
    } else if (item.ends_with("seven")) {
      return 7;
    } else if (item.ends_with("eight")) {
      return 8;
    } else if (item.ends_with("nine")) {
      return 9;
    }
    item.pop_back();
  }
  return -1;
}

int part_2(vector<string> input) {
  vector<int> ints;

  for (string item : input) {
    int start = get_start_int(item);
    int end = get_end_int(item);

    int output = start * 10 + end;
    ints.push_back(output);
  }

  int sum{0};
  for (int item : ints) {
    sum += item;
  }
  return sum;
}

int main() {

  vector<string> lines = read_input_file("01");

  print_part_1(part_1(lines));
  print_part_2(part_2(lines));

  return 0;
}
