//
// Created by TSO1841 on 11/24/2021.
//

#ifndef AOC_2021_PERSON_H
#define AOC_2021_PERSON_H
#include <string>

struct Person {
    std::string nameString();

    Person(std::string name, int age);

private:
    std::string name;
    int age;
};


#endif //AOC_2021_PERSON_H
