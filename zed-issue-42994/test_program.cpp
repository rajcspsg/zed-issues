#include <iostream>
#include <unistd.h>
#include <thread>
#include <chrono>

int main() {
    std::cout << "Debug test program started (PID: " << getpid() << ")" << std::endl;
    std::cout << "This program will run for 60 seconds..." << std::endl;

    for (int i = 1; i <= 60; i++) {
        std::cout << "Second " << i << " of 60" << std::endl;
        std::this_thread::sleep_for(std::chrono::seconds(1));
    }

    std::cout << "Program completed normally" << std::endl;
    return 0;
}
