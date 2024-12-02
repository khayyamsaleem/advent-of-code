LIB_OUTPUT_DIR = ../../build/lib
MODULE_NAME = solve02
SRC = solve.cpp
CXX = zig c++
CXXFLAGS = -shared -fPIC -std=c++17 -g
BINFLAGS = -std=c++17 -Wall -Wextra -g

all:
	mkdir -p $(LIB_OUTPUT_DIR)
	$(CXX) $(CXXFLAGS) solve.cpp -o $(LIB_OUTPUT_DIR)/lib$(MODULE_NAME).so

test:
	$(CXX) $(BINFLAGS) -o $(MODULE_NAME) $(SRC)
	./$(MODULE_NAME)


clean:
	rm -f $(LIB_OUTPUT_DIR)/lib$(MODULE_NAME).so
	rm -f $(MODULE_NAME)

