#pragma once
#include <enet/enet.h>
#include "PlayerState.hpp"
#include <array>
#include "../core/Logger.hpp"

class NetworkManager {
    ENetHost* server = nullptr;
    std::array<PlayerState, 8> playerStates;
public:
    NetworkManager();
    ~NetworkManager();

    bool StartServer(unsigned short port = 1234, int maxClients = 8);
    void SendPlayerState(const PlayerState& state);
    void Update();
    const std::array<PlayerState, 8>& GetPlayerStates() const;
};