# Toolchain file compiling for aarch64 macOS

set(triple arm64-apple-macos12)
set(arch arm64)
set(HACL_TARGET_OS osx)
set(CMAKE_C_COMPILER clang)
set(CMAKE_C_COMPILER_TARGET ${triple})
set(CMAKE_CXX_COMPILER clang++)
set(CMAKE_CXX_COMPILER_TARGET ${triple})
# This isn't working unfortunately. It's being set in CMakeLists.txt again
set(CMAKE_SYSTEM_PROCESSOR ${arch})
set(CMAKE_OSX_ARCHITECTURES ${arch})
