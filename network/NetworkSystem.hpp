#pragma once
#include "../math/Vector3.hpp"
#include <vector>
#include <string>
#include <functional>

class NetworkClient {
private:
    std::string address;
    int port;
    bool connected;
    
public:
    NetworkClient(const std::string& addr, int p);
    ~NetworkClient();
    
    bool Connect();
    void Disconnect();
    bool IsConnected() const { return connected; }
    
    void SendData(const std::string& data);
    void ReceiveData();
    
    std::string GetAddress() const { return address; }
    int GetPort() const { return port; }
};

class NetworkServer {
private:
    int port;
    bool running;
    std::vector<NetworkClient*> clients;
    
public:
    NetworkServer(int p);
    ~NetworkServer();
    
    bool Start();
    void Stop();
    bool IsRunning() const { return running; }
    
    void Update();
    void Broadcast(const std::string& data);
    
    size_t GetClientCount() const { return clients.size(); }
};

class NetworkSystem {
private:
    NetworkServer* server;
    NetworkClient* client;
    bool isServer;
    bool enabled;
    
public:
    NetworkSystem();
    ~NetworkSystem();
    
    bool StartServer(int port = 1234);
    bool ConnectToServer(const std::string& address, int port = 1234);
    void Disconnect();
    
    void Update();
    void SendPlayerState(const Vector3& position, const Vector3& velocity);
    void ReceivePlayerStates();
    
    bool IsServer() const { return isServer; }
    bool IsConnected() const;
    bool IsEnabled() const { return enabled; }
    
    void SetEnabled(bool enable) { enabled = enable; }
};