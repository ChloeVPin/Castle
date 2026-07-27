// A multi-file Castle project using the opt-in cmake backend.
// Build with:  castle build --backend cmake
// Run with:    castle run
#include "greet.hpp"

int main() {
    greet("multi-file castle");
    return 0;
}
