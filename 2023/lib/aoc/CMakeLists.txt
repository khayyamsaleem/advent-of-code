add_library(aoc SHARED get_input.cpp)

find_package(CURL REQUIRED)
target_link_libraries(aoc PRIVATE CURL::libcurl)

target_include_directories(aoc
    PUBLIC 
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/..> # Add this line
    $<INSTALL_INTERFACE:include>
)
