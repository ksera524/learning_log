package projava;


import java.util.Arrays;
import java.util.List;
import java.util.stream.Collectors;

public class ForEachArraySample {
    public static void main(String[] args) {
        var nums = new int[]{2,3,5,7};
        Arrays.stream(nums).forEach(num -> System.out.println(num));

        var names = new String[]{"yusuke","kis","sugiyama"};
        var longNames = Arrays.stream(names).filter(name -> name.length() >= 5).collect(Collectors.toList());
        System.out.println(longNames);

        var strs = List.of("apple","banana","orange","pineapple");
        strs.stream().filter(s -> s.length() > 5).map(s -> s.toUpperCase()).forEach(s -> System.out.println(s));
        strs.stream().filter(s -> s.length() > 5).map(s -> s.length()).count();

        System.out.println(strs.stream().filter(str -> str.length() > 5).mapToInt(s -> s.length()).sum());
        strs.stream().allMatch(s -> s.contains("a"));
        strs.stream().anyMatch(s -> s.contains("c"));


    }
}
