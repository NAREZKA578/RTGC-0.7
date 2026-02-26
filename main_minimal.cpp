#include "EngineMinimal.hpp"
#include <iostream>

int main() {
    Engine engine;
    
    std::cout << "Initializing RTGC Engine..." << std::endl;
    
    if (!engine.Initialize()) {
        std::cout << "Failed to initialize RTGC Engine" << std::endl;
        return -1;
    }
    
    engine.Run();
    
    return 0;
}
