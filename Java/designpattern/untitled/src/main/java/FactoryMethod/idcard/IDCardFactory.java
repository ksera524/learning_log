package FactoryMethod.idcard;

import FactoryMethod.framework.Factory;
import FactoryMethod.framework.Product;

public class IDCardFactory extends Factory {
    private int idNumber = 100;
    @Override
    protected Product createProduct(String owner){
        return new IDCard(owner,idNumber++);
    }

    @Override
    protected void registerProduct(Product product){
        System.out.println(product + "を登録しました");
    }
}
