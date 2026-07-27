cmake_minimum_required(VERSION 3.20)
project(multi LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# Castle: gather every translation unit under src/.
file(GLOB_RECURSE CASTLE_SOURCES CONFIGURE_DEPENDS src/*.cpp)
add_executable(${PROJECT_NAME} ${CASTLE_SOURCES})
