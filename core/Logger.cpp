#include "Logger.hpp"

// Инициализация статических переменных
std::mutex Logger::logMutex;
std::ofstream Logger::logFile;
bool Logger::fileOpened = false;
bool Logger::consoleEnabled = true;
