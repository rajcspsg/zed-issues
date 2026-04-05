CXX = g++
CXXFLAGS = -g -std=c++11
TARGET = test_program
SRC = test_program.cpp

all: $(TARGET)

$(TARGET): $(SRC)
	$(CXX) $(CXXFLAGS) -o $(TARGET) $(SRC)

clean:
	rm -f $(TARGET)

.PHONY: all clean
