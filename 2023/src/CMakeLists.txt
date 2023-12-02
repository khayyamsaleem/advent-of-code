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

add_library(day1_lib STATIC day1.cpp)
target_include_directories(day1_lib PUBLIC ../lib/)

add_executable(aoc2023 main.cpp)
target_link_libraries(aoc2023 PRIVATE libaoc)
target_link_libraries(aoc2023 PRIVATE cpp_dotenv)
target_link_libraries(aoc2023 PRIVATE day1_lib)