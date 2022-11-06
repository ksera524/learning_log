package AbstractFuctory;

import AbstractFuctory.factory.Factory;
import AbstractFuctory.factory.Link;
import AbstractFuctory.factory.Page;
import AbstractFuctory.factory.Tray;

import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        Scanner scan = new Scanner(System.in);
        String filename = scan.nextLine();
        String classname = scan.nextLine();
        scan.close();

        Factory factory = Factory.getFactory(classname);

        Link blog1 = factory.createLink("Blog 1","https://example.com/blog1");
        Link blog2 = factory.createLink("Blog 2","https://example.com/blog2");
        Link blog3 = factory.createLink("Blog 3","https://example.com/blog3");

        Tray blogTray = factory.createTray("Blog Site");
        blogTray.add(blog1);
        blogTray.add(blog2);
        blogTray.add(blog3);

        Link news1 = factory.createLink("News 1","https://example.com/news1");
        Link news2 = factory.createLink("News 2","https://example.com/news2");
        Tray news3 = factory.createTray("News 3");
        news3.add(factory.createLink("News3(US)","https://examle.com/news3us"));
        news3.add(factory.createLink("News3(JP)","https://examle.com/news3jp"));

        Tray newsTray = factory.createTray("News Site");
        newsTray.add(news1);
        newsTray.add(news2);
        newsTray.add(news3);

        Page page = factory.createPage("Blog and News","Hitoshi YUki");
        page.add(blogTray);
        page.add(newsTray);

        page.output(filename);
    }
}
