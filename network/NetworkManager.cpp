#include "NetworkManager.hpp"
#include <cstring> // for memcpy

NetworkManager::NetworkManager() {
    std::fill(playerStates.begin(), playerStates.end(), PlayerState{});
}

bool NetworkManager::StartServer(unsigned short port, int maxClients) {
    enet_initialize();
    ENetAddress addr;
    addr.host = ENET_HOST_ANY;
    addr.port = port;
    server = enet_host_create(&addr, maxClients, 2, 0, 0);
    if (!server) {
        ERROR(L"Сервер не запущен");
        return false;
    }
    LOG(L"Сетевой сервер запущен");
    return true;
}

void NetworkManager::SendPlayerState(const PlayerState& state) {
    ENetPacket* packet = enet_packet_create(&state, sizeof(state), ENET_PACKET_FLAG_RELIABLE);
    if (packet) {
        enet_host_broadcast(server, 0, packet);
    }
}

void NetworkManager::Update() {
    ENetEvent event;
    while (enet_host_service(server, &event, 0) > 0) {
        if (event.type == ENET_EVENT_TYPE_RECEIVE) {
            if (event.packet->dataLength == sizeof(PlayerState)) {
                PlayerState s;
                std::memcpy(&s, event.packet->data, sizeof(PlayerState));
                if (s.playerId < playerStates.size()) {
                    playerStates[s.playerId] = s;
                }
            }
            enet_packet_destroy(event.packet);
        }
    }
}

const std::array<PlayerState, 8>& NetworkManager::GetPlayerStates() const {
    return playerStates;
}

NetworkManager::~NetworkManager() {
    if (server) enet_host_destroy(server);
    enet_deinitialize();
}