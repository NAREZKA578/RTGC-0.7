#include "NetworkSystem.hpp"
#include "../core/Logger.hpp"

NetworkClient::NetworkClient(const std::string& addr, int p) : address(addr), port(p), connected(false) {
}

NetworkClient::~NetworkClient() {
    Disconnect();
}

bool NetworkClient::Connect() {
    // В реальной реализации здесь было бы подключение по TCP/UDP
    connected = true;
    Logger::Log("Подключено к серверу: ", address, ":", port);
    return true;
}

void NetworkClient::Disconnect() {
    if (connected) {
        connected = false;
        Logger::Log("Отключено от сервера: ", address, ":", port);
    }
}

void NetworkClient::SendData(const std::string& data) {
    if (!connected) return;
    Logger::Log("Отправка данных: ", data);
}

void NetworkClient::ReceiveData() {
    if (!connected) return;
    // В реальной реализации здесь было бы получение данных
}

NetworkServer::NetworkServer(int p) : port(p), running(false) {
}

NetworkServer::~NetworkServer() {
    Stop();
}

bool NetworkServer::Start() {
    // В реальной реализации здесь был бы запуск сервера
    running = true;
    Logger::Log("Сервер запущен на порту: ", port);
    return true;
}

void NetworkServer::Stop() {
    if (running) {
        running = false;
        for (auto client : clients) {
            delete client;
        }
        clients.clear();
        Logger::Log("Сервер остановлен");
    }
}

void NetworkServer::Update() {
    if (!running) return;
    
    // В реальной реализации здесь была бы обработка новых подключений
    // и прием данных от клиентов
}

void NetworkServer::Broadcast(const std::string& data) {
    if (!running) return;
    Logger::Log("Рассылка данных ", clients.size(), " клиентам: ", data);
}

NetworkSystem::NetworkSystem() : server(nullptr), client(nullptr), isServer(false), enabled(false) {
}

NetworkSystem::~NetworkSystem() {
    Disconnect();
}

bool NetworkSystem::StartServer(int port) {
    if (server) return false;
    
    server = new NetworkServer(port);
    if (server->Start()) {
        isServer = true;
        enabled = true;
        Logger::Log("NetworkSystem запущен как сервер");
        return true;
    } else {
        delete server;
        server = nullptr;
        return false;
    }
}

bool NetworkSystem::ConnectToServer(const std::string& address, int port) {
    if (client) return false;
    
    client = new NetworkClient(address, port);
    if (client->Connect()) {
        isServer = false;
        enabled = true;
        Logger::Log("NetworkSystem подключен к серверу");
        return true;
    } else {
        delete client;
        client = nullptr;
        return false;
    }
}

void NetworkSystem::Disconnect() {
    if (server) {
        server->Stop();
        delete server;
        server = nullptr;
    }
    
    if (client) {
        client->Disconnect();
        delete client;
        client = nullptr;
    }
    
    isServer = false;
    enabled = false;
    Logger::Log("NetworkSystem отключен");
}

void NetworkSystem::Update() {
    if (!enabled) return;
    
    if (isServer && server) {
        server->Update();
    } else if (!isServer && client) {
        client->ReceiveData();
    }
}

void NetworkSystem::SendPlayerState(const Vector3& position, const Vector3& velocity) {
    if (!enabled) return;
    
    std::string data = "PLAYER_STATE:" + 
        std::to_string(position.x) + "," + 
        std::to_string(position.y) + "," + 
        std::to_string(position.z) + "|" +
        std::to_string(velocity.x) + "," + 
        std::to_string(velocity.y) + "," + 
        std::to_string(velocity.z);
    
    if (isServer && server) {
        server->Broadcast(data);
    } else if (!isServer && client) {
        client->SendData(data);
    }
}

void NetworkSystem::ReceivePlayerStates() {
    // В реальной реализации здесь был бы прием и обработка состояний других игроков
}

bool NetworkSystem::IsConnected() const {
    if (isServer) {
        return server && server->IsRunning();
    } else {
        return client && client->IsConnected();
    }
}