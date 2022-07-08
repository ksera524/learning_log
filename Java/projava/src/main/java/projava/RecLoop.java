package projava;public class RecLoop {
    public static void main(String[] args) {
        loop(0);
        loop_2(3);

    }
    static void loop(int i){
        if (i >=5){
            return;
        }
        System.out.println(i);
        loop(i+1);
    }
    static void loop_2(int i){
        if (i <= 0){
            return;
        }
        System.out.println(i);
        loop_2(i - 1);
    }
}
