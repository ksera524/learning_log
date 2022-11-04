package Prototype;

import Prototype.framework.Maneger;
import Prototype.framework.Product;

public class Main {
    public static void main(String[] args) {
        Maneger maneger = new Maneger();
        UnderlinePen upen = new UnderlinePen('-');
        MessageBox mbox = new MessageBox('*');
        MessageBox sbox = new MessageBox('/');

        maneger.register("strong message",upen);
        maneger.register("warning box",mbox);
        maneger.register("slash box",sbox);

        Product p1 = maneger.create("strong message");
        p1.use("Hello world");

        Product p2 = maneger.create("warning box");
        p2.use("Hello World");

        Product p3 = maneger.create("slash box");
        p3.use("Hello World");
    }
}
