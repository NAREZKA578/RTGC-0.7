CC = gcc
CFLAGS = -Wall -Wextra -std=c99 -O2
LIBS = -L./raylib/src -lraylib -lGL -lm -lpthread -ldl -lrt
INCLUDES = -I./raylib/src

TARGET = game
SOURCES = main.c physics_engine_c.c vehicle_control.c

all: $(TARGET)

$(TARGET): $(SOURCES)
	$(CC) $(CFLAGS) $(INCLUDES) -o $(TARGET) $(SOURCES) $(LIBS)

clean:
	rm -f $(TARGET)

.PHONY: all clean