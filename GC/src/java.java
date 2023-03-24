class Person {
    int age;

    Person(int n) {
        this.age = n;
    } 
}

public class java {
    static int SIZE = 1_000_000;
    static int LOOPS = 1000;

    public static void fill_mem(Person[] array) {
        for (int i = 0; i < array.length; i++) {
            array[i] = new Person(i);
        } 
    }

    public static void update_mem(Person[] array) {
        for (int i = 0; i < array.length; i++) {
            array[i].age += 1;
        } 
    }

    public static void main(String[] args) {
        long pid = ProcessHandle.current().pid();
        System.out.println("Java pid: " + pid);
        System.out.println("Total iterations: " + SIZE*LOOPS);

        while (true) {
            long start = System.nanoTime();

            //--------------------------------
            Person[] array = new Person[SIZE];
            fill_mem(array);
            for (int i = 0; i < LOOPS; i++) {
                update_mem(array);
            }    
            //--------------------------------

            long end = System.nanoTime();
            double duration = (end - start)/(1_000_000_000.0);
            System.out.println(String.format("%.5f", duration));
        }
    }


}


