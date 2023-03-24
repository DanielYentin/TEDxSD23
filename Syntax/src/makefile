python: run_python
java: compile_java run_java
c: compile_c run_c
rust: compile_rust run_rust

run_python:
	@python3 python.py
run_java:
	@java -Xmx1024M -Xms1024M java 
run_c:
	@./c.out
run_rust:
	@./rust.out

compile_all: compile_c compile_java compile_rust

compile_c:
	@gcc c.c -O -o c.out
compile_java:
	@javac java.java
compile_rust:
	@rustc rust.rs -O -o rust.out

clean:
	@rm java.class; rm c.out; rm rust.out
	