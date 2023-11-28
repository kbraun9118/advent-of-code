#include "lib.hpp"
#include <fstream>
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

  return output;
}
