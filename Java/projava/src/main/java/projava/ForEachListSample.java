package projava;

import java.util.List;

public class ForEachListSample {
    public static void main(String[] args) {
        var strs = List.of("apple","banana","grape");
        for (String str : strs) {
            System.out.println(str);
        }
    }
}
