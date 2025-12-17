# Hecksagon Interpreter Makefile (Rust, cross-platform, /usr/opt/hecksagon on Linux)

# Compiler and flags
RUSTC = rustc
RUSTFLAGS = -O

# Detect OS
ifeq ($(OS),Windows_NT)
    EXE_EXT = .exe
    INSTALL_DIR = C:/opt/hecksagon
    RM = powershell -Command "Remove-Item -Force -ErrorAction SilentlyContinue '$(TARGET)'"
    MKDIR = powershell -Command "if (-Not (Test-Path '$(INSTALL_DIR)')) { New-Item -ItemType Directory -Path '$(INSTALL_DIR)' }"
    COPY = powershell -Command "Copy-Item -Path"
else
    EXE_EXT =
    INSTALL_DIR = /usr/opt/hecksagon
    RM = rm -f
    MKDIR = sudo mkdir -p
    COPY = sudo cp
endif

# Target executable
TARGET = hecksagon$(EXE_EXT)

# Source file
SRC = hecksagon.rs

# Default rule: compile
all: $(TARGET)

$(TARGET): $(SRC)
	$(RUSTC) $(RUSTFLAGS) -o $(TARGET) $(SRC)

# Install target
install: $(TARGET)
	$(MKDIR)           # <- no extra argument
ifeq ($(OS),Windows_NT)
	$(COPY) $(TARGET) "$(INSTALL_DIR)/$(TARGET)"
	@echo "Installed $(TARGET) to $(INSTALL_DIR)."
	@echo "Please restart your terminal or run '$env:Path += \"$(INSTALL_DIR)\"' to use 'hecksagon' in this session."
else
	$(COPY) $(TARGET) "$(INSTALL_DIR)/$(TARGET)"
	@echo "Installed $(TARGET) to $(INSTALL_DIR)."
endif
	$(MAKE) clean


# Clean up
# Clean up... Wait i said that twice 😭

clean:
ifeq ($(OS),Windows_NT)
	powershell -Command "if (Test-Path '$(TARGET)') { Remove-Item -Force '$(TARGET)' }"
else
	$(RM)
endif

# Update: clean, rebuild, and install
update: # Yay now the potato is hot 
	git pull origin main 
	$(MAKE) clean
	$(MAKE) install
