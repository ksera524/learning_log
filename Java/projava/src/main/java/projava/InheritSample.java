package projava;

import java.util.List;

public class InheritSample {
    static abstract class User{
        String name;

        User(String name){
            this.name = name;
        }
        public String getName(){
            return this.name;
        }

        @Override
        public String toString() {
            return profile();
        }

        abstract String profile();
    }
    static class Student extends User{
        int score;
        Student(String name,int score){
            super(name);
            this.score = score;
        }

        public int getScore(){
            return this.score;
        }

        @Override
        String profile() {
            return "学生 %s,%d点".formatted(getName(),getScore());
        }
    }
    static class Teacher extends User{
        String subject;
        Teacher(String name,String subject){
            super(name);
            this.subject = subject;
        }
        public String getSubject(){
            return this.subject;
        }

        @Override
        String profile() {
            return "先生 %s,教科 %s".formatted(getName(),getSubject());
        }
    }
    public static void main(String[] args) {
        List<User> people = List.of(new Student("kis",80),
                new Teacher("hosoya","Math"),
                new User("匿名"){
                    @Override
                    String profile() {
                        return "ダミー";
                    }
                });
        for (var p:people){
            System.out.println("こんにちは%sさん".formatted(p.getName()));
            System.out.println(p.toString());
        }
    }
}
