package FactoryMethod.idcard;

import FactoryMethod.framework.Product;



public class IDCard extends Product {
    private final String owner;
    private final int idNumber;

    IDCard(String owner,int idNumber){
        System.out.println(owner + "のカードを作ります");
        System.out.println("No." + idNumber);
        this.owner = owner;
        this.idNumber = idNumber;
    }

    @Override
    public void use(){
        System.out.println(this + "を使います");
    }

    @Override
    public String toString(){
        return "[IDCard: " + owner + "]";
    }

    public String getOwner(){
        return owner;
    }
}
