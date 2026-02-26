#pragma once
#include <glm/glm.hpp>
#include <vector>
#include <string>
#include <unordered_map>
#include "../core/Logger.hpp"

struct OSMNode {
    double lat, lon;
    float x, z;
};

class OSMParser {
    static double latToY(double lat);
    static double lonToX(double lon);
public:
    static std::vector<std::pair<glm::vec3, glm::vec3>> ParseRoadsFromXML(const std::string& xmlData);
};