#pragma once
#include "../math/Vector3.hpp"
#include <vector>
#include <string>

class AudioSound {
private:
    std::string filename;
    bool loaded;
    float volume;
    bool looping;
    
public:
    AudioSound(const std::string& file);
    ~AudioSound();
    
    bool Load();
    void Play();
    void Stop();
    void Pause();
    void Resume();
    
    void SetVolume(float vol) { volume = vol; }
    float GetVolume() const { return volume; }
    void SetLooping(bool loop) { looping = loop; }
    bool IsLooping() const { return looping; }
    bool IsLoaded() const { return loaded; }
};

class AudioSystem {
private:
    std::vector<AudioSound*> sounds;
    bool enabled;
    float masterVolume;
    
public:
    AudioSystem();
    ~AudioSystem();
    
    bool Initialize();
    void Shutdown();
    void Update();
    
    AudioSound* LoadSound(const std::string& filename);
    void PlaySound(const std::string& filename);
    void StopSound(const std::string& filename);
    
    void SetMasterVolume(float volume) { masterVolume = volume; }
    float GetMasterVolume() const { return masterVolume; }
    void SetEnabled(bool enable) { enabled = enable; }
    bool IsEnabled() const { return enabled; }
    
    void StopAllSounds();
    size_t GetSoundCount() const { return sounds.size(); }
};