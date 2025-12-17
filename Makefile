# Hecksagon Interpreter Makefile (Rust version)

# Compiler and flags
RUSTC = rustc
RUSTFLAGS = -O

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
SRC = hecksagon.rs

# Default rule: compile
all: $(TARGET)

$(TARGET): $(SRC)
	$(RUSTC) $(RUSTFLAGS) -o $(TARGET) $(SRC)

# Install target
install: $(TARGET)
	$(MKDIR)
	$(COPY) $(TARGET) "$(INSTALL_DIR)/$(TARGET)"
	@echo "Installed $(TARGET) to $(INSTALL_DIR)"

# Clean up
clean:
	$(RM) $(TARGET)

.PHONY: all install clean
