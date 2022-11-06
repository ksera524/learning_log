package Bridge;

public class StringDisplayImpl extends DisplayImpl{
    private final String string;
    private final int width;

    public StringDisplayImpl(String string){
        this.string = string;
        this.width = string.length();
    }

    @Override
    public void rawOpen() {
        printLine();
    }

    @Override
    public void rawPrint() {
        System.out.println("|" + string + "|");
    }

    @Override
    public void rawClose() {
        printLine();
    }

    public void printLine(){
        System.out.print("+");
        System.out.print("-".repeat(width));
        System.out.println("+");
    }
}
