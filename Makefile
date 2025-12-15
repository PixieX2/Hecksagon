# Hecksagon Interpreter Makefile

# Compiler and flags
CXX = g++
CXXFLAGS = -O2 -std=c++17

# Target executable
TARGET = hecksagon

# Source files
SRC = hecksagon.cpp

# Default rule: compile
all: $(TARGET)

$(TARGET): $(SRC)
	$(CXX) $(CXXFLAGS) -o $(TARGET) $(SRC)

# Clean up
clean:
	rm -f $(TARGET)
