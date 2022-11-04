package Prototype;

import Prototype.framework.Product;

public class UnderlinePen implements Product {
    private final char ulchar;
    public UnderlinePen(char ulchar){
        this.ulchar = ulchar;
    }

    public UnderlinePen(UnderlinePen prototype){
        this.ulchar = prototype.ulchar;
    }

    @Override
    public void use(String s){
        System.out.println(s);
        for(int i = 0;i<s.length();i++){
            System.out.print(ulchar);
        }
        System.out.println();
    }

    @Override
    public Product createCopy(){
        return new UnderlinePen(this);
    }
}
