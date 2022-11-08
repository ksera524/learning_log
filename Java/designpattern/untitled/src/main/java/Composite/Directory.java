package Composite;

import java.util.ArrayList;
import java.util.List;

public class Directory extends Entry{
    private final String name;
    private List<Entry> directory = new ArrayList<>();

    public Directory(String name){
        this.name = name;
    }

    @Override
    public String getName() {
        return this.name;
    }

    @Override
    public int getSIze() {
        int size = 0;
        for (Entry e:directory){
            size += e.getSIze();
        }
        return size;
    }

    @Override
    protected void printList(String prefix) {
        System.out.println(prefix + "/" + this);
        for (Entry e:directory){
            e.printList(prefix + "/" + name);
        }
    }

    public Entry add (Entry entry){
        directory.add(entry);
        return this;
    }
}
