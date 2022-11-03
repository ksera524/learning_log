package SingletonPractice;

import java.util.Arrays;
import java.util.HashMap;
import java.util.Map;

public class Triple {
    private static Map<String,Triple> map = new HashMap<>();
    static {
        String[] name = {"ALPHA","BETA","GAMMA"};
        Arrays.stream(name).forEach(s -> map.put(s,new Triple(s)));
    }

    private String name;

    private Triple(String name){
        System.out.println("name is " + name);
        this.name = name;
    }

    public static Triple getInstance(String name){
        return map.get(name);
    }
}
