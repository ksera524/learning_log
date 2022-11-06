package Bridge;

import java.util.Random;

public class RandomDisplay extends Display{
    public RandomDisplay(DisplayImpl impl){
        super(impl);
    }

    public void randumDisplay(){
        Random rand = new Random();
        open();
        for (int i = 0; i < rand.nextInt(10); i++){
            print();
        }
        close();
    }
}
