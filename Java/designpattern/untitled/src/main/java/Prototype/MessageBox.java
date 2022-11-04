package Prototype;

import Prototype.framework.Product;

public class MessageBox implements Product {
    private final char decochar;

    public MessageBox(char ch){
        this.decochar = ch;
    }

    public MessageBox(MessageBox prototype){
        this.decochar = prototype.decochar;
    }

    @Override
    public void use(String s){
        for (int i = 0;i<s.length() + 2;i++){
            System.out.print(decochar);
        }
        System.out.println();
        System.out.println(decochar + s + decochar);
        for (int i = 0;i<s.length() + 2;i++){
            System.out.print(decochar);
        }
        System.out.println();
    }

    @Override
    public Product createCopy(){
        return new MessageBox(this);
    }
}
