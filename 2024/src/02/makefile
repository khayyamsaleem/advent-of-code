OUTPUT_DIR = ../../build/lib
MODULE_NAME = solve02

CXX = zig c++
CXXFLAGS = -shared -fPIC -std=c++17 -g

all:
	mkdir -p $(OUTPUT_DIR)
	$(CXX) $(CXXFLAGS) solve.cpp -o $(OUTPUT_DIR)/lib$(MODULE_NAME).so

clean:
	rm -f $(OUTPUT_DIR)/lib$(MODULE_NAME).so

