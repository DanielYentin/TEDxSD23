public class java {
    public static void main(String[] args) {
        int a = 0;
        int b = 10;
        int sum = sum_evens(a, b);
        System.out.println("Java: sum of even numbers between " + a + " and " + b + " is " + sum);
    }

    public static int sum_evens(int a, int b) {
        if (a <= b) {
            int sum = 0;
            for (int i = a; i <= b; i++) {
                if (i % 2 == 0) {
                    sum += i;
                }
            }
            return sum;
        } 
        else {
            return 0;
        }
    }
}