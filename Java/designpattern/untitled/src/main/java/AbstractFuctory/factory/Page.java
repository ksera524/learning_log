package AbstractFuctory.factory;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.StandardOpenOption;
import java.util.ArrayList;
import java.util.List;

public abstract class Page {
    protected final String title;
    protected final String auther;
    protected final List<Item> content = new ArrayList<>();

    public Page(String title,String auther){
        this.title = title;
        this.auther = auther;
    }

    public void add(Item item){
        content.add((item));
    }

    public void output(String filename){
        try {
            Files.writeString(Path.of(filename),makeHTML(),
                    StandardOpenOption.CREATE,
                    StandardOpenOption.TRUNCATE_EXISTING,
                    StandardOpenOption.WRITE);
            System.out.println(filename + "を作成しました");
        } catch (IOException e){
            e.printStackTrace();
        }
    }
    public abstract String makeHTML();
}
