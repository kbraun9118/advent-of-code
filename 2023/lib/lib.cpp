#include "lib.hpp"
#include <fstream>
#include <iostream>
#include <string>
#include <tuple>

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

vector<string> split_string(string input, string seperator) {
  auto rest = input;
  vector<string> output;
  while (!rest.empty()) {
    int pos = rest.find(seperator);
    if (pos == string::npos) {
      output.push_back(rest);
      break;
    }
    output.push_back(rest.substr(0, pos));
    rest = rest.substr(pos + seperator.length(),
                       rest.length() - pos - seperator.length() + 1);
  }
  return output;
}

tuple<string, string> split_string_once(string input, string seperator) {
  auto split = split_string(input, seperator);
  if (split.size() < 2) {
    throw runtime_error{"Cannot split once"};
  }

  string first = split[0];

  string second = "";
  for (int i = 1; i < split.size(); i++) {
    second += split[i];
    if (i < split.size() - 1) {
      second += seperator;
    }
  }

  return tuple{first, second};
}

// trim from start (in place)
void ltrim(std::string &s) {
  s.erase(s.begin(), std::find_if(s.begin(), s.end(), [](unsigned char ch) {
            return !std::isspace(ch);
          }));
}

// trim from end (in place)
void rtrim(std::string &s) {
  s.erase(std::find_if(s.rbegin(), s.rend(),
                       [](unsigned char ch) { return !std::isspace(ch); })
              .base(),
          s.end());
}

// trim from both ends (in place)
void trim(std::string &s) {
  rtrim(s);
  ltrim(s);
}

// trim from start (copying)
std::string ltrim_copy(std::string s) {
  ltrim(s);
  return s;
}

// trim from end (copying)
std::string rtrim_copy(std::string s) {
  rtrim(s);
  return s;
}

// trim from both ends (copying)
std::string trim_copy(std::string s) {
  trim(s);
  return s;
}

void print_part_1(std::string output) { cout << "Part 1: " << output << endl; }
void print_part_2(std::string output) { cout << "Part 2: " << output << endl; }
void print_part_1(int output) { cout << "Part 1: " << output << endl; }
void print_part_2(int output) { cout << "Part 2: " << output << endl; }
