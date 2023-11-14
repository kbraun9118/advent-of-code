//
// Created by TSO1841 on 11/24/2021.
//

#include <iostream>
#include <lib/person/Person.h>

int main() {
    Person person { "Hello", 30 };
    std::cout << person.nameString() << std::endl;
    return 0;
}