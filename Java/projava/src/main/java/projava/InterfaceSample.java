package projava;

import java.util.List;

public class InterfaceSample {
    @FunctionalInterface
    interface Named{
        String name();
        default String greeting(){
            return "こんにちは%sさん".formatted(name());
        }
    }
    record Student(String name,int score) implements Named{};
    record Teacher(String name,String subject) implements Named{};
    static void message(Named named){
        System.out.println("Hello " + named.name());
    }

    public static void main(String[] args) {
        var people = List.of(new Student("kis",80),new Teacher("Hosoya","Math"));
        for(var p:people){
            System.out.println("%s".formatted(p.name()));
        }

       message(() -> "No name");
    }
}
