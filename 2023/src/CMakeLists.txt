include(FetchContent)

## puzzle dependencies
find_package(Boost REQUIRED)

## puzzle linking
file(GLOB PUZZLE_SOURCES "day*.cpp")
add_library(puzzle STATIC ${PUZZLE_SOURCES})
target_link_libraries(puzzle PRIVATE
  Boost::boost
)

## main dependencies

### dotenv
FetchContent_Declare(dotenv GIT_REPOSITORY https://github.com/khayyamsaleem/cpp-dotenv.git GIT_TAG v1.0.1-alpha)
FetchContent_GetProperties(dotenv)
if(NOT dotenv_POPULATED)
  FetchContent_Populate(dotenv)
endif()
add_subdirectory(${dotenv_SOURCE_DIR} ${dotenv_BINARY_DIR})

## main linking
add_executable(aoc2023 main.cpp)
target_link_libraries(aoc2023 PRIVATE
  libaoc
  cpp_dotenv
  puzzle
)
