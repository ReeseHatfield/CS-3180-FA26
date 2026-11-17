import java.util.ArrayList;
import java.util.Arrays;

public class Driver {

    
    public static void main(String[] args){
        System.out.println("hi");

        ArrayList<String> list = new ArrayList<>(Arrays.asList(
            "Alice", "Bob", "Charlie"
        ));

        // printFirstAndObj(list, "Another String");

        ArrayList<String> names = new ArrayList<>();
	    names.add("Ada");
	 
	    String result = firstOrDefault(names, "Default");
	    System.out.println(result.toUpperCase());
    }


    public static <T> T firstOrDefault(
	        ArrayList<T> items, T fallback) {
	    if (items.isEmpty()) {
	        return fallback;
	    }
	 
	    T first = items.get(0);
	    return first;
	}

    // public static <T> void printFirstAndObj(ArrayList<T> list, Object o){
	//     T first = list.get(0);
	 
	//     System.out.println("First of list was: " + first);
	//     System.out.println("Object was: " + o);
	 
	//     if(o instanceof T){
	//         System.out.println("T was the same type as o");
	//     }
	 
	// }
}