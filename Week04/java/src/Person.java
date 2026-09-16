public class Person extends Object {
    private String name;
    private double gpa;

    public Person(String name, double gpa){
        this.name = name;
        this.gpa = gpa;
    }

    


    @Override
    public String toString(){
        return this.name + " with a " + this.gpa;
    }
}
