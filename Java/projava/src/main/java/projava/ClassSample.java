package projava;

import java.util.Objects;

public class ClassSample {
    static final class Student {
        private final String name;
        private final int score;

        Student(String name, int score) {
            this.name = name;
            this.score = score;
        }

        Student(String name){
            this(name,0);
        }

        Student(int score){
            this("no name",score);
        }

        Student(){
            this("no name");
        }

        public String name() {
            return name;
        }

        public int score() {
            return score;
        }

        @Override
        public boolean equals(Object obj) {
            if (obj == this) return true;
            if (obj == null || obj.getClass() != this.getClass()) return false;
            var that = (Student) obj;
            return Objects.equals(this.name, that.name) &&
                    this.score == that.score;
        }

        @Override
        public int hashCode() {
            return Objects.hash(name, score);
        }

        @Override
        public String toString() {
            return "Student[" +
                    "name=" + name + ", " +
                    "score=" + score + ']';
        }
    }

    public static void main(String[] args) {
        var s = new Student("kis" ,89);
        System.out.println(s);
        System.out.println(s.score);
    }
}
