package Singleton;

public class Singleton {
    private static final Singleton singleton = new Singleton();
    private Singleton(){
        System.out.println("インスタンスを作成しました");
    }

    public static Singleton getInstance(){
        return singleton;
    }
}
