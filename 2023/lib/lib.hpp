#pragma once

#include <string>
#include <vector>

std::vector<std::string> read_input_file(std::string day);
std::vector<std::string> read_test_file(std::string day);
std::vector<std::string> split_string(std::string input, std::string seperator);
std::tuple<std::string, std::string> split_string_once(std::string input, std::string seperator);
void trim(std::string &s);
std::string trim_copy(std::string s);
void print_part_1(std::string output);
void print_part_2(std::string output);
void print_part_1(int output);
void print_part_2(int output);
