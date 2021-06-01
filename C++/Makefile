all: generate_seed

CPPFLAGS=-I./ 

#Exclude the library directories
SOURCES := $(shell find . -name '*.cpp')
# Objs are all the sources, with .cpp replaced by .o
OBJS := $(SOURCES:.cpp=.o)

.PHONY: server clean


generate_seed: $(OBJS)
	g++ -g $(CPPFLAGS) $(OBJS) ../target/debug/libbip39.a -lpthread -lstdc++ -Wl,--no-as-needed -ldl -o generate_seed
	chmod 755 generate_seed



.cpp.o:
	g++ -o $@ $(CPPFLAGS) $(CFLAGS) -c $<

#ALL_DEPS := $(patsubst %.o,%.h,$(MAIN_OBS))
#-include $(ALL_DEPS)
	
clean:
	rm -rf *~ generate_seed
	find ./ -name "*.o" | xargs -n 1 rm -f
	find ./ -name "*.d" | xargs -n 1 rm -f
	