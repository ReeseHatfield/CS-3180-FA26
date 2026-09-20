public class Truck implements Drivable, Ride {

    @Override
    public void driveToWork() {
        System.out.println("Drives to work with a truck");
    }

   

    @Override
    public void driveToSchool() {
        System.out.println("Drives to school with a truck");
    }
    
}
