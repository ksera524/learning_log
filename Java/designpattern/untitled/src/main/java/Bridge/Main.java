package Bridge;

public class Main {
    public static void main(String[] args) {
        Display d1 = new Display(new StringDisplayImpl("Hello Japan."));
        Display d2 = new Display(new StringDisplayImpl("Hello World."));
        CountDisplay d3 = new CountDisplay(new StringDisplayImpl("Hello Universe."));
        RandomDisplay d4 = new RandomDisplay(new StringDisplayImpl("Test"));
        d1.display();
        d2.display();
        d3.multiDisplay(3);
        d4.randumDisplay();

        IncreaseDisplay d5 = new IncreaseDisplay(new CharDisplayImpl('<','*','>'),1);
        IncreaseDisplay d6 = new IncreaseDisplay(new CharDisplayImpl('|','#','-'),3);

        d5.increaseDisplay(4);
        d6.increaseDisplay(6);
    }
}
