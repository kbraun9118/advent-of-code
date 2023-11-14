//
// Created by TSO1841 on 11/24/2021.
//

#include <string>
#include "Person.h"

std::string Person::nameString() {
    return "Name: " + name + ", Age: " + std::to_string(age);
}

Person::Person(std::string name, int age) : name(std::move(name)), age(age) {}
