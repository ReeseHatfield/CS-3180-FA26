public class Driver {
    public static void main(String[] args) {
        Person p = new Person("David", 1.9);
        
        // System.out.println(p.toString());

        System.out.println();
        System.out.println();
        foo(p);
    }


    public static void foo(Object o){
        // lookup the impl at runtime
        System.out.println(o.toString());
    }
}