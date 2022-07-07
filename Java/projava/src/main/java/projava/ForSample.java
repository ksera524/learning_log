package projava;

public class ForSample {
    public static void main(String[] args) {
        for(int i = 0;i<5;i++){
            System.out.println(i);
        }

        int i = 0;
        while (i < 5){
            System.out.println(i);
            i++;
        }

        for(i = 1;i<10;i++){
            for(int j = 1;j<10;j++){
                System.out.printf("%2d|",i*j);
            }
            System.out.println("");
        }
    }
}
