#pragma once
#include <iostream>
#include <string>
#include <sstream>
#include <mutex>
#include <chrono>
#include <iomanip>
#include <fstream>
#include <ctime>
#include <thread>

class Logger {
private:
    static std::mutex logMutex;
    static std::ofstream logFile;
    static bool fileOpened;
    static bool consoleEnabled;
    
    static void EnsureLogFile() {
        if (fileOpened) return;
        
        try {
            auto now = std::chrono::system_clock::now();
            auto time = std::chrono::system_clock::to_time_t(now);
            std::tm tm;
#ifdef _WIN32
            localtime_s(&tm, &time);
#else
            localtime_r(&time, &tm);
#endif
            
            std::ostringstream filename;
            filename << "logs/log_"
                    << std::put_time(&tm, "%Y%m%d_%H%M%S")
                    << ".txt";
            
            logFile.open(filename.str(), std::ios::out | std::ios::app);
            if (logFile.is_open()) {
                fileOpened = true;
                logFile << "===== Лог начат " << std::put_time(&tm, "%Y-%m-%d %H:%M:%S") << " =====" << std::endl;
            } else {
                std::cerr << "Не удалось открыть файл лога: " << filename.str() << std::endl;
            }
        } catch (const std::exception& e) {
            std::cerr << "Ошибка при создании лог-файла: " << e.what() << std::endl;
        }
    }
    
    static std::string GetTimestamp() {
        auto now = std::chrono::system_clock::now();
        auto time = std::chrono::system_clock::to_time_t(now);
        std::tm tm;
#ifdef _WIN32
        localtime_s(&tm, &time);
#else
        localtime_r(&time, &tm);
#endif
        
        std::ostringstream oss;
        oss << std::put_time(&tm, "%H:%M:%S");
        return oss.str();
    }
    
    static std::string GetThreadID() {
        auto id = std::this_thread::get_id();
        std::ostringstream oss;
        oss << id;
        return oss.str();
    }
    
    template<typename T>
    static void LogMessage(const std::string& level, const T& message) {
        std::lock_guard<std::mutex> lock(logMutex);
        
        std::ostringstream oss;
        oss << "[" << GetTimestamp() << "] [" << GetThreadID() << "] [" << level << "] " << message;
        
        if (consoleEnabled) {
            if (level == "ОШИБКА") {
                std::cerr << oss.str() << std::endl;
            } else {
                std::cout << oss.str() << std::endl;
            }
        }
        
        EnsureLogFile();
        if (fileOpened) {
            logFile << oss.str() << std::endl;
            logFile.flush();
        }
    }
    
public:
    static void EnableConsole(bool enable) { consoleEnabled = enable; }
    static void Flush() {
        std::lock_guard<std::mutex> lock(logMutex);
        if (fileOpened) {
            logFile.flush();
        }
    }
    
    template<typename... Args>
    static void Log(const Args&... args) {
        std::ostringstream oss;
        (oss << ... << args);
        LogMessage("ИНФО", oss.str());
    }
    
    template<typename... Args>
    static void Warning(const Args&... args) {
        std::ostringstream oss;
        (oss << ... << args);
        LogMessage("ПРЕДУПРЕЖДЕНИЕ", oss.str());
    }
    
    template<typename... Args>
    static void Error(const Args&... args) {
        std::ostringstream oss;
        (oss << ... << args);
        LogMessage("ОШИБКА", oss.str());
    }
    
    template<typename... Args>
    static void Debug(const Args&... args) {
        std::ostringstream oss;
        (oss << ... << args);
        LogMessage("ОТЛАДКА", oss.str());
    }
    
    static void Close() {
        std::lock_guard<std::mutex> lock(logMutex);
        if (fileOpened) {
            auto now = std::chrono::system_clock::now();
            auto time = std::chrono::system_clock::to_time_t(now);
            std::tm tm;
#ifdef _WIN32
            localtime_s(&tm, &time);
#else
            localtime_r(&time, &tm);
#endif
            
            logFile << "===== Лог завершен " << std::put_time(&tm, "%Y-%m-%d %H:%M:%S") << " =====" << std::endl;
            logFile.close();
            fileOpened = false;
        }
    }
};
