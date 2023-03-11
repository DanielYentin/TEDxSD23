public class java {
    static int TESTS = 5_000_000_0;

    public static void main(String[] args) {
        long pid = ProcessHandle.current().pid();
        System.out.println("Java pid: " + pid);

        while (true) {
            long start = System.nanoTime();

            Person[] array = new Person[TESTS];
            fill_mem(array);
            
            long end = System.nanoTime();

            // convert nano to sec
            double duration = (end - start)/(1_000_000_000.0);
            System.out.println(String.format("%.5f", duration));
            System.out.println(String.format("%.5f", duration));
        }
    }

    public static void fill_mem(Person[] array) {
        for (int i = 0; i < array.length; i++) {
            array[i] = new Person(i);
        } 
    }
}

class Person {
    int age;

    Person(int n) {
        this.age = n;
    } 
}
