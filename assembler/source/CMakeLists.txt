# Minimum version of CMake required
cmake_minimum_required(VERSION 3.10)

# Project name
project(PoliAs)

# Set the C++ standard
set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED True)

# Get all source files in the current directory
file(GLOB SOURCES "*.cpp" "*.h")

# Add the executable
add_executable(PoliAs ${SOURCES})

# Optionally, you can specify the main file explicitly
# add_executable(MyProject main.cpp ${SOURCES})
