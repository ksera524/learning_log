package projava;


public class MethodSample {
    record Student(String name,int englishScore,int mathScore){
        int average(){
            return (this.englishScore() + this.mathScore())/2;
        }

        public int maxScore() {
            return Math.max(englishScore(),mathScore());
        }
    };
    public static void main(String[] args) {
        var result = twice(3);
        System.out.println(result);

        var kis = new Student("kis",60,80);
        var a = kis.average();
        System.out.println(a);

        int maxScore = kis.maxScore();
        System.out.println(maxScore);
    }
    static int twice(int x){
        return x*2;
    }

}
