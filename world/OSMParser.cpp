#include "OSMParser.hpp"
#include <sstream>
#include <string>
#include <cmath> // для cos

double OSMParser::latToY(double lat) { return lat * 111000.0; }
double OSMParser::lonToX(double lon) { return lon * 111000.0 * cos(0.0); } // Упрощённо, cos(широта в радианах)

std::vector<std::pair<glm::vec3, glm::vec3>> OSMParser::ParseRoadsFromXML(const std::string& xmlData) {
    std::unordered_map<int, OSMNode> nodes;
    std::vector<std::pair<glm::vec3, glm::vec3>> roads;
    size_t pos = 0;

    // Парсинг узлов
    while ((pos = xmlData.find("<node id=\"", pos)) != std::string::npos) {
        int id = std::stoi(xmlData.substr(pos + 10));
        size_t latPos = xmlData.find("lat=\"", pos);
        size_t lonPos = xmlData.find("lon=\"", pos);
        if (latPos == std::string::npos || lonPos == std::string::npos) break;
        double lat = std::stod(xmlData.substr(latPos + 5));
        double lon = std::stod(xmlData.substr(lonPos + 5));
        nodes[id] = {lat, lon, static_cast<float>(lonToX(lon)), static_cast<float>(latToY(lat))};
        pos += 20;
    }

    // Парсинг путей
    pos = 0;
    while ((pos = xmlData.find("<way", pos)) != std::string::npos) {
        size_t endWay = xmlData.find("</way>", pos);
        std::string way = xmlData.substr(pos, endWay - pos + 6);
        if (way.find("highway") == std::string::npos) {
            pos = endWay;
            continue;
        }
        std::vector<int> refs;
        size_t ndPos = 0;
        while ((ndPos = way.find("<nd ref=\"", ndPos)) != std::string::npos) {
            int ref = std::stoi(way.substr(ndPos + 9));
            refs.push_back(ref);
            ndPos += 20;
        }
        for (size_t i = 0; i < refs.size() - 1; ++i) {
            auto it1 = nodes.find(refs[i]);
            auto it2 = nodes.find(refs[i + 1]);
            if (it1 != nodes.end() && it2 != nodes.end()) {
                roads.emplace_back(
                    glm::vec3(it1->second.x, 0, it1->second.z),
                    glm::vec3(it2->second.x, 0, it2->second.z)
                );
            }
        }
        pos = endWay;
    }
    LOG(L"Дороги из OSM загружены");
    return roads;
}