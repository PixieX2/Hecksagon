# Hecksagon Interpreter Makefile

# Compiler and flags
CXX = g++
CXXFLAGS = -O2 -std=c++17

# Detect OS
ifeq ($(OS),Windows_NT)
    RM = del /Q
    EXE_EXT = .exe
    INSTALL_DIR = C:/Program Files/Hecksagon
    MKDIR = if not exist "$(INSTALL_DIR)" mkdir "$(INSTALL_DIR)"
    COPY = copy
else
    RM = rm -f
    EXE_EXT =
    INSTALL_DIR = /usr/local/bin
    MKDIR = mkdir -p
    COPY = cp
endif

# Target executable
TARGET = hecksagon$(EXE_EXT)

# Source files
SRC = hecksagon.cpp

# Default rule: compile
all: $(TARGET)

$(TARGET): $(SRC)
	$(CXX) $(CXXFLAGS) -o $(TARGET) $(SRC)

# Install target
install: $(TARGET)
	$(MKDIR)
	$(COPY) $(TARGET) "$(INSTALL_DIR)/$(TARGET)"
	@echo "Installed $(TARGET) to $(INSTALL_DIR)"

# Clean up
clean:
	$(RM) $(TARGET)
