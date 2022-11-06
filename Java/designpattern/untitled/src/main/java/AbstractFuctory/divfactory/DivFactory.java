package AbstractFuctory.divfactory;

import AbstractFuctory.factory.Factory;
import AbstractFuctory.factory.Link;
import AbstractFuctory.factory.Page;
import AbstractFuctory.factory.Tray;

public class DivFactory  extends Factory {
    @Override
    public Link createLink(String caption,String url){
        return new DivLink(caption,url);
    }

    @Override
    public Tray createTray(String caption) {
        return new DivTray(caption);
    }

    @Override
    public Page createPage(String title, String author) {
        return new DivPage(title,author);
    }
}
