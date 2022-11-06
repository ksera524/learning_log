package AbstractFuctory.listfactory;

import AbstractFuctory.factory.Factory;
import AbstractFuctory.factory.Link;
import AbstractFuctory.factory.Page;
import AbstractFuctory.factory.Tray;

public class ListFactory extends Factory {
    @Override
    public Link createLink(String caption,String url){
        return new ListLink(caption,url);
    }

    @Override
    public Tray createTray(String caption){
        return new ListTray(caption);
    }

    @Override
    public Page createPage(String title, String author){
        return new ListPage(title,author);
    }
}
