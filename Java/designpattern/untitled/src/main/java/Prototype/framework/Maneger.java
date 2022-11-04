package Prototype.framework;

import java.util.HashMap;
import java.util.Map;

public class Maneger {
    private Map<String,Product> showcase = new HashMap<>();

    public void register(String name,Product product){
        showcase.put(name,product);
    }

    public Product create(String productName){
        Product p = showcase.get(productName);
        return p.createCopy();
    }
}
