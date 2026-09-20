public class Driver {
    public static void main(String[] args) {
        Person p = new Person("David", 1.9);
        
        // System.out.println(p.toString());

        System.out.println();
        System.out.println();
        foo(p);

        Drivable truck = new Truck();
        Drivable car = new Car();
    
        commute(truck);
        // commute(car);
    }


    public static void foo(Object o){
        // lookup the impl at runtime
        System.out.println(o.toString());
    }

    public static void commute(Drivable d){
        d.driveToWork();
    }
}