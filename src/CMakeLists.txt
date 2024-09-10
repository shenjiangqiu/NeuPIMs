CMAKE_MINIMUM_REQUIRED(VERSION 3.23)
include(FetchContent)

FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG v0.5 # Optionally specify a commit hash, version tag or branch here
)
# Set any global configuration variables such as `Rust_TOOLCHAIN` before this line!
FetchContent_MakeAvailable(Corrosion)

corrosion_import_crate(MANIFEST_PATH "rust/neupimrust/Cargo.toml")
# Project setting
set(LIB_NAME "Simulator")

# Find all source files except main.cc for the library
file(GLOB_RECURSE LIB_SRC_FILES
  "${CMAKE_SOURCE_DIR}/src/*.h"
  "${CMAKE_SOURCE_DIR}/src/*.cc"
)

# Exclude main.cc from the library source files
list(REMOVE_ITEM LIB_SRC_FILES "${CMAKE_SOURCE_DIR}/src/main.cc")

# Find only main.cc for the executable
set(MAIN_FILE "${CMAKE_SOURCE_DIR}/src/main.cc")

# Find external packages
find_package(robin_hood REQUIRED)

# Add the library with all source files except main.cc
add_library(${LIB_NAME}_lib ${LIB_SRC_FILES})

# Link Conan libraries to the lib (defined in conanfile.txt)
find_package(Boost REQUIRED)
find_package(spdlog REQUIRED)
find_package(nlohmann_json REQUIRED)

# Add include directories and link libraries for the lib
target_include_directories(${LIB_NAME}_lib PUBLIC ${CMAKE_SOURCE_DIR}/src/ rust/neupimrust)

# Link external libraries to the library
target_link_libraries(${LIB_NAME}_lib PUBLIC robin_hood::robin_hood boost::boost spdlog::spdlog nlohmann_json::nlohmann_json dramsim3 booksim2 neupimrust)

# Add the executable with only the main.cc and link to the lib
add_executable(${LIB_NAME} ${MAIN_FILE})
target_link_libraries(${LIB_NAME} PUBLIC ${LIB_NAME}_lib)



