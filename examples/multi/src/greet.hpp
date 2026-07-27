#pragma once
#include <string>
#include <print>

// A tiny greeting helper shared across translation units.
inline void greet(const std::string& who) {
    std::println("Hello, {}! Your castle stands.", who);
}
