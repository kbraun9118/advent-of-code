#include "lib.hpp"
#include <fstream>
#include <iostream>
#include <string>

using namespace std;

vector<string> read_input_file(string day) {
  ifstream input_file{"../input/day_" + day + "/input.txt"};
  string line;
  vector<string> output;
  if (input_file.is_open()) {
    while (getline(input_file, line)) {
      output.push_back(line);
    }
    input_file.close();
  }

  if (output[output.size() - 1] == "") {
    output.pop_back();
  }

  return output;
}

vector<string> read_test_file(string day) {
  ifstream input_file{"../input/day_" + day + "/test.txt"};
  string line;
  vector<string> output;
  if (input_file.is_open()) {
    while (getline(input_file, line)) {
      output.push_back(line);
    }
    input_file.close();
  }

  if (output[output.size() - 1] == "") {
    output.pop_back();
  }

  return output;
}

void print_part_1(std::string output) { cout << "Part 1: " << output << endl; }
void print_part_2(std::string output) { cout << "Part 2: " << output << endl; }
void print_part_1(int output) { cout << "Part 1: " << output << endl; }
void print_part_2(int output) { cout << "Part 1: " << output << endl; }
