import java.awt.Color;
import java.lang.reflect.Constructor;
import java.lang.reflect.Field;
import java.lang.reflect.Method;

class Dog extends Object {
    public int ageInDogYears = 0;
    private String name;
    private Color color;
	    
    public Dog(String name, Color c){
        this.ageInDogYears = 0;
        this.name = name;
        this.color = c;
    }

    public Dog(Dog other){
        this.ageInDogYears = other.ageInDogYears;
        this.name = other.name;
        this.color = other.color;
    }

    private void playFetch(){
        System.out.println(this.name + "Plays fetch");
    }

    @Override
    public String toString(){
        return this.name + ", " + this.ageInDogYears + ", " + this.color;
    }
	 
}

class Reflection {
    public static void main(String[] args) throws Exception {
        // Dog d1 = new Dog();


        // Class<?> clazz = d1.getClass();

        String input = "Dog";

        Class<?> clazz = Class.forName(input);

        Constructor<?> constr = clazz.getDeclaredConstructor(
            String.class,
            Color.class
        );

        Dog instance = (Dog) constr.newInstance("Clifford", Color.RED);
        instance.ageInDogYears = 34;

        // Method m = clazz.getMethod("playFetch");
        // String result = (String) m.invoke(instance);

        // System.out.println(result);

        Field f = clazz.getDeclaredField("color");
        f.setAccessible(true);
        Color c = (Color) f.get(instance);
        System.out.println(c);

        // System.out.println(f.get(instance));


        // System.out.println(dog);

    }
}