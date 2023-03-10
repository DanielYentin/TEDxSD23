public class java {
    static int tests = 5_000_000;
    public static void main(String[] args) {
        long pid = ProcessHandle.current().pid();
        System.out.println("Java pid: " + pid);

        while (true) {
            long start = System.nanoTime();
            Person[] array = fill_mem(tests);
            long end = System.nanoTime();
            
            // convert nano to sec
            double duration = (end - start)/(1_000_000_000.0);
            System.out.println(String.format("%.3f", duration));
        }
    }

    public static Person[] fill_mem(int n) {
        Person[] list = new Person[n];
        for (int i = 0; i < list.length; i++) {
            list[i] = new Person(i);
        } 
        return list;
    }
}

class Person {
    int age;

    Person(int n) {
        this.age = n;
    } 
}