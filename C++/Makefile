all: debug release

CPPFLAGS=-I./ 

#Exclude the library directories
SOURCES := $(shell find . -name '*.cpp')
# Objs are all the sources, with .cpp replaced by .o
OBJS := $(SOURCES:.cpp=.o)

.PHONY: server clean


debug: $(OBJS)
	g++ -g $(CPPFLAGS) $(OBJS) ../target/debug/libbip39.a -lpthread -lstdc++ -Wl,--no-as-needed -ldl -o generate_seed_debug
	chmod 755 generate_seed_debug

release: $(OBJS)
	g++ $(CPPFLAGS) $(OBJS) ../target/release/libbip39.a -lpthread -lstdc++ -Wl,--no-as-needed -ldl -o generate_seed_release
	chmod 755 generate_seed_release	



.cpp.o:
	g++ -o $@ $(CPPFLAGS) $(CFLAGS) -c $<

#ALL_DEPS := $(patsubst %.o,%.h,$(MAIN_OBS))
#-include $(ALL_DEPS)
	
clean:
	rm -rf *~ generate_seed*
	find ./ -name "*.o" | xargs -n 1 rm -f
	find ./ -name "*.d" | xargs -n 1 rm -f
	