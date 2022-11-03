package TemplateMethod;

public class CharDisplay implements AbstractDisplay{
    private final char ch;

    public CharDisplay(char ch){
        this.ch = ch;
    }

    @Override
    public void open(){
        System.out.print("<<");
    }

    @Override
    public void print(){
        System.out.print(ch);
    }

    @Override
    public void close(){
        System.out.println(">>");
    }
}
