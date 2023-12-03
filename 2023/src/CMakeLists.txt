include(FetchContent)

FetchContent_Declare(
    dotenv
    GIT_REPOSITORY https://github.com/khayyamsaleem/cpp-dotenv.git
    GIT_TAG v1.0.1-alpha
)

FetchContent_GetProperties(dotenv)
if(NOT dotenv_POPULATED)
    FetchContent_Populate(dotenv)
endif()
add_subdirectory(${dotenv_SOURCE_DIR} ${dotenv_BINARY_DIR})

file(GLOB PUZZLE_SOURCES "day*.cpp")

add_library(puzzle_lib STATIC ${PUZZLE_SOURCES})
target_include_directories(puzzle_lib PUBLIC ../lib/)

add_executable(aoc2023 main.cpp)
target_link_libraries(aoc2023 PRIVATE libaoc)
target_link_libraries(aoc2023 PRIVATE cpp_dotenv)
target_link_libraries(aoc2023 PRIVATE puzzle_lib)
