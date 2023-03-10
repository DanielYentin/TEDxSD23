all:
	@make python
	@make java
	@make c
	@make rust

c:
	@make compile_c && make run_c
java:
	@make compile_java && make run_java
python: 
	@make run_python
rust:
	@make compile_rust && make run_rust

run_c:
	@./c.out
run_java:
	# limit memory of program to 256mb to force GC to run more frequently
	@java -Xmx256M -Xms256M java 
run_python:
	@python3 python.py
run_rust:
	@./rust.out

compile_all:
	@compile_c
	@compile_java
	@compile_rust

compile_c:
	@gcc c.c -o c.out
compile_java:
	@javac java.java
compile_rust:
	@rustc rust.rs -o rust.out

clean:
	@rm c.out; rm java.class; rm Person.class; rm rust.out;