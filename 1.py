import os
import sys

def create_project_structure(base_path):
    # Список всех файлов для создания
    files = [
        "src/main.cpp",
        "src/core/Engine.hpp",
        "src/core/Engine.cpp",
        "src/core/ECSManager.hpp",
        "src/core/ECSManager.cpp",
        "src/core/Logger.hpp",
        "src/core/SnapshotSystem.hpp",
        "src/core/SnapshotSystem.cpp",
        "src/graphics/Renderer.hpp",
        "src/graphics/Renderer.cpp",
        "src/graphics/Shader.hpp",
        "src/graphics/Shader.cpp",
        "src/graphics/Camera.hpp",
        "src/graphics/Camera.cpp",
        "src/graphics/ThirdPersonCamera.hpp",
        "src/graphics/ThirdPersonCamera.cpp",
        "src/graphics/FirstPersonCamera.hpp",
        "src/graphics/FirstPersonCamera.cpp",
        "src/graphics/Mesh.hpp",
        "src/graphics/Mesh.cpp",
        "src/graphics/RenderableVehicle.hpp",
        "src/graphics/RenderableVehicle.cpp",
        "src/graphics/LightingSystem.hpp",
        "src/graphics/LightingSystem.cpp",
        "src/graphics/ShadowMap.hpp",
        "src/graphics/ShadowMap.cpp",
        "src/graphics/PostProcessingSystem.hpp",
        "src/graphics/PostProcessingSystem.cpp",
        "src/graphics/AnimationSystem.hpp",
        "src/graphics/AnimationSystem.cpp",
        "src/physics/PhysicsUpdateSystem.hpp",
        "src/physics/PhysicsUpdateSystem.cpp",
        "src/physics/PhysXInitializer.hpp",
        "src/physics/PhysXInitializer.cpp",
        "src/physics/CharacterController.hpp",
        "src/physics/CharacterController.cpp",
        "src/network/NetworkManager.hpp",
        "src/network/NetworkManager.cpp",
        "src/network/PlayerState.hpp",
        "src/network/NetworkSyncSystem.hpp",
        "src/network/NetworkSyncSystem.cpp",
        "src/network/SpawnSystem.hpp",
        "src/network/SpawnSystem.cpp",
        "src/world/CityGenerator.hpp",
        "src/world/CityGenerator.cpp",
        "src/world/RoadNetwork.hpp",
        "src/world/RoadNetwork.cpp",
        "src/world/Terrain.hpp",
        "src/world/Terrain.cpp",
        "src/audio/AudioSystem.hpp",
        "src/audio/AudioSystem.cpp",
        "src/audio/AudioEventManager.hpp",
        "src/audio/AudioEventManager.cpp",
        "src/audio/FootstepSystem.hpp",
        "src/audio/FootstepSystem.cpp",
        "src/audio/AmbientSystem.hpp",
        "src/audio/AmbientSystem.cpp",
        "src/ui/HudUI.hpp",
        "src/ui/HudUI.cpp",
        "src/ui/MenuSystem.hpp",
        "src/ui/MenuSystem.cpp",
        "src/ui/InventoryUI.hpp",
        "src/ui/InventoryUI.cpp",
        "src/game/GameLevel.hpp",
        "src/game/GameLevel.cpp",
        "src/game/VehicleType.hpp",
        "src/game/VehicleType.cpp",
        "src/game/VehicleFactory.hpp",
        "src/game/VehicleFactory.cpp",
        "src/game/Vehicle.hpp",
        "src/game/Vehicle.cpp",
        "src/game/InputManager.hpp",
        "src/game/InputManager.cpp",
        "src/game/BuildingSystem.hpp",
        "src/game/BuildingSystem.cpp",
        "src/game/WeatherSystem.hpp",
        "src/game/WeatherSystem.cpp",
        "src/game/Inventory.hpp",
        "src/game/Inventory.cpp",
        "src/game/DamageSystem.hpp",
        "src/game/DamageSystem.cpp",
        "src/game/WeaponSystem.hpp",
        "src/game/WeaponSystem.cpp",
        "src/game/InteractionSystem.hpp",
        "src/game/InteractionSystem.cpp",
        "src/game/QuestSystem.hpp",
        "src/game/QuestSystem.cpp",
        "src/game/ProgressionSystem.hpp",
        "src/game/ProgressionSystem.cpp",
        "src/components/TransformComponent.hpp",
        "src/components/VehicleComponent.hpp",
        "src/components/RenderableComponent.hpp",
        "src/components/CharacterComponent.hpp",
        "src/components/BuildingComponent.hpp",
        "src/components/InventoryComponent.hpp",
        "src/systems/RenderSystem.hpp",
        "src/systems/RenderSystem.cpp",
        "src/systems/PhysicsUpdateSystem.hpp",
        "src/systems/PhysicsUpdateSystem.cpp",
        "src/systems/CharacterSystem.hpp",
        "src/systems/CharacterSystem.cpp",
        "src/systems/NetworkSyncSystem.hpp",
        "src/systems/NetworkSyncSystem.cpp",
        "src/systems/AIController.hpp",
        "src/ai/AIController.hpp",
        "src/ai/AIController.cpp",
        "src/cuda/WindCuda.cu",
        "src/cuda/WindCuda.h",
        "src/cuda/SuspensionCuda.cu",
        "src/cuda/SuspensionCuda.h",
        "src/cuda/TractionCuda.cu",
        "src/cuda/TractionCuda.h",
        "src/cuda/TerrainCuda.cu",
        "src/cuda/TerrainCuda.h",
        "src/math/Vector3.hpp",
        "src/math/Mass.hpp",
        "src/math/PhysicsUtils.hpp",
        "src/objects/GameObject.hpp",
        "src/objects/GameObject.cpp",
        "src/objects/RenderableObject.hpp",
        "src/objects/RenderableObject.cpp",
        "src/objects/PhysicsObject.hpp",
        "src/objects/PhysicsObject.cpp",
        "src/objects/AudioObject.hpp",
        "src/objects/AudioObject.cpp",
        "src/platform/PlatformAbstraction.hpp",
        "src/platform/PlatformAbstraction.cpp",
        "src/debug/Profiler.hpp",
        "src/debug/Profiler.cpp",
        "assets/models/kamaz.obj",
        "assets/shaders/vertex.glsl",
        "assets/shaders/fragment.glsl",
        "CMakeLists.txt",
        "build.bat"
    ]

    # Создаём корневую директорию проекта
    project_root = os.path.join(base_path, "RTGC")
    os.makedirs(project_root, exist_ok=True)

    # Создаём все файлы
    created_files = 0
    for file_path in files:
        full_path = os.path.join(project_root, file_path)
        os.makedirs(os.path.dirname(full_path), exist_ok=True)
        
        # Создаём пустой файл только если он не существует
        if not os.path.exists(full_path):
            with open(full_path, 'w') as f:
                # Для некоторых файлов добавляем базовое содержимое
                if file_path == "CMakeLists.txt":
                    f.write("# CMakeLists.txt for RTGC project\n")
                elif file_path.endswith((".cpp", ".hpp", ".h", ".cu")):
                    f.write("// Auto-generated file\n")
                elif file_path.endswith((".glsl", ".obj")):
                    f.write("# Auto-generated asset file\n")
            created_files += 1
    
    return project_root, created_files, len(files) - created_files

def main():
    print("="*50)
    print("RTGC Project Structure Generator")
    print("="*50)
    
    # Запрашиваем путь у пользователя
    default_path = os.path.join(os.getcwd(), "RTGC")
    print(f"\nПо умолчанию проект будет создан в: {default_path}")
    
    while True:
        user_input = input("\nВведите желаемый путь (или нажмите Enter для пути по умолчанию):\n> ").strip()
        
        if not user_input:
            base_path = os.getcwd()
            break
            
        # Раскрываем ~ для home директории (для Unix-систем)
        base_path = os.path.expanduser(user_input)
        
        # Проверяем существование пути
        if not os.path.exists(base_path):
            create_choice = input(f"Директория '{base_path}' не существует. Создать её? (y/n): ").lower()
            if create_choice == 'y':
                try:
                    os.makedirs(base_path, exist_ok=True)
                    break
                except Exception as e:
                    print(f"Ошибка при создании директории: {e}")
                    continue
            else:
                print("Пожалуйста, укажите существующий путь.")
                continue
        
        if not os.path.isdir(base_path):
            print("Указанный путь не является директорией. Пожалуйста, попробуйте снова.")
            continue
        
        # Проверяем права на запись
        if not os.access(base_path, os.W_OK):
            print("Нет прав на запись в указанную директорию. Пожалуйста, выберите другой путь.")
            continue
        
        break
    
    # Подтверждение перед созданием
    project_path = os.path.join(base_path, "RTGC")
    print(f"\n{'='*50}")
    print(f"Проект будет создан в: {project_path}")
    confirm = input("Продолжить? (y/n): ").lower()
    
    if confirm != 'y':
        print("\nОперация отменена пользователем.")
        sys.exit(0)
    
    # Создание структуры
    print("\nСоздание файловой структуры...")
    try:
        created_path, new_files, existing_files = create_project_structure(base_path)
        print(f"\n{'='*50}")
        print("✅ Структура успешно создана!")
        print(f"📁 Путь: {created_path}")
        print(f"🆕 Создано новых файлов: {new_files}")
        if existing_files > 0:
            print(f"ℹ️  Пропущено существующих файлов: {existing_files}")
        print(f"\nПерейдите в директорию проекта: cd {os.path.basename(created_path)}")
    except Exception as e:
        print(f"\n❌ Ошибка при создании структуры: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()