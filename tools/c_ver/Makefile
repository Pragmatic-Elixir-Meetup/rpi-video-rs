CC=gcc

INCLUDE_DIR = /opt/vc/include
LIB_DIR = /opt/vc/lib
LIBS = -lmmal_core -lmmal_util -lmmal_vc_client -lvcos

CFLAGS = -I$(INCLUDE_DIR)
LDFLAGS = -L$(LIB_DIR) $(LIBS)

TARGET = rpi_video.out
OBJS = recorder.o

%.o: %.c
	$(CC) -c -o $@ $< $(CFLAGS)

$(TARGET): $(OBJS)
	$(CC) -o $@ $^ $(LDFLAGS)

.PHONY: clean

clean:
	rm -f $(TARGET) $(OBJS)
