NAME=main

.PHONY: all clean

all: ${NAME}

${NAME}: ${NAME}.rs
	rustc $< 

clean:
	rm -f ${NAME}
