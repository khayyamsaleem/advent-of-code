find_package(GTest CONFIG REQUIRED)
enable_testing()

add_executable(test_runner
  test.cpp
)

target_include_directories(
  test_runner
  PRIVATE
  ../src
)

target_link_libraries(
  test_runner
  PRIVATE
  GTest::gtest
  GTest::gtest_main
  puzzle_lib
)

add_test(NAME all_tests COMMAND test_runner)
