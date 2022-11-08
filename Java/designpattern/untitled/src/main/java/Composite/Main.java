package Composite;


public class Main {
    public static void main(String[] args) {
        System.out.println("Making root");
        Directory rootDir = new Directory("root");
        Directory binDir = new Directory("bin");
        Directory tempDir = new Directory("temp");
        Directory useDir = new Directory("use");
        rootDir.add(binDir);
        rootDir.add(tempDir);
        rootDir.add(useDir);
        binDir.add(new File("vi" ,10000));
        binDir.add(new File("latex",20000));
        rootDir.printList();
    }
}
