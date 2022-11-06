package Bridge;

public class IncreaseDisplay extends CountDisplay{
    private final int step;

    public IncreaseDisplay(DisplayImpl impl,int step){
        super(impl);
        this.step = step;
    }

    public void increaseDisplay(int level){
        int count = 0;
        for(int i=0;i<level;i++){
            multiDisplay(count);
            count += step;
        }
    }
}
