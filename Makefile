CFLAGS=-std=c11 -g -static

#ここの依存関係判断はcargoに任せる
test: 
	./test.sh

clean:
	rm -f nineccr *.o *~ tmp*

.PHONY: test clean