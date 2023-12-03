cmake_minimum_required(VERSION 3.20)
project(aoc_2021)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_BUILD_TYPE Debug)
# set(CMAKE_CXX_COMPILER Clang++)

include_directories(.)

add_subdirectory(lib)

add_subdirectory(day_01)
add_subdirectory(day_02)
