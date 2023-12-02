add_library(libaoc SHARED get_input.cpp)

find_package(CURL REQUIRED)
target_link_libraries(libaoc PRIVATE CURL::libcurl)

target_include_directories(libaoc
    PUBLIC 
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/..> # Add this line
    $<INSTALL_INTERFACE:include>
)