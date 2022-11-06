package AbstractFuctory.listfactory;

import AbstractFuctory.factory.Item;
import AbstractFuctory.factory.Page;

public class ListPage extends Page {
    public ListPage(String title,String author){
        super(title,author);
    }

    @Override
    public String makeHTML(){
        StringBuilder sb = new StringBuilder();
        sb.append("<!DOCTYPE html>\n");
        sb.append("<html>\n");
        sb.append("<head><title>");
        sb.append(title);
        sb.append("</title></head>\n");
        sb.append("<body>\n");
        sb.append("<h1>");
        sb.append(title);
        sb.append("</h1>\n");
        sb.append("<ul>\n");
        for(Item item: content){
            sb.append(item.makeHTML());
        }
        sb.append("</ul>\n");
        sb.append("<hr><address>");
        sb.append(auther);
        sb.append("</address>\n");
        sb.append("<body></html>\n");
        return sb.toString();
    }
}
