package TemplateMethod;

public class Main {
    public static void main(String[] args) {
        AbstractDisplay ch = new CharDisplay('X');
        ch.display();

        AbstractDisplay sd = new StringDisplay("Hello World");
        sd.display();
    }
}
