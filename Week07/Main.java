import java.util.Arrays;
import java.util.List;
import java.util.stream.Collectors;

public class Main {
    public static void main(String[] args){
	List<String> names = Arrays.asList(
	    "John", "Sarah", "Jack", "Emily"
	);
	 
	List<String> result = names.stream()
	    .filter(name -> name.startsWith("J"))
	    .map((s) -> {
            return s.toUpperCase();
        }) //wrap this method call in a lambda
	    .collect(Collectors.toList());
	 
	System.out.println(result);
    }
}