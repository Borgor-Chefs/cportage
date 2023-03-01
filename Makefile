FLAGS	:= -g -Wall -Werror
LDARGS:= -lncurses -ltinfo
CXX		:= gcc
SRCS	:= *.c
OBJDIR	:= objs
OBJECTS	:= $(SRCS:%.c=$(OBJDIR)/%.o)
PROG	:= out

.PHONY: all clean

all: $(PROG)

$(OBJDIR):
	mkdir -p $@

$(OBJDIR)/%.o: %.c | $(OBJDIR)
	$(CXX) $(FLAGS) -c $< -o $@

$(PROG): $(OBJECTS) | $(OBJDIR)
	$(CXX) $(LDARGS) $^ -o $@

clean:
	@rm -rfv $(OBJDIR) $(PROG)

