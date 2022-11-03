package TemplateMethod;

public class StringDisplay implements AbstractDisplay{
    private final String string;
    private final int width;

    public StringDisplay(String string){
        this.string = string;
        this.width = string.length();
    }

    public void printLine(){
        System.out.print("+");
        System.out.print("-".repeat(width));
        System.out.println("+");
    }
    @Override
    public void open(){
        printLine();
    }

    @Override
    public void print(){
        System.out.println("|" + string + "|");
    }

    @Override
    public void close(){
        printLine();
    }
}
