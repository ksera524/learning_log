package Bridge;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;

public class FileDisplayImpl extends DisplayImpl{
    private final String filename;


    public FileDisplayImpl(String filename){
        this.filename = filename;
    }

    @Override
    public void rawOpen() {
        System.out.println(filename);
    }

    @Override
    public void rawPrint() {
        try {
            for (String line: Files.readAllLines(Path.of(filename))){
                System.out.println("> ");
                System.out.println(line);
            }
        } catch (IOException e){
            e.printStackTrace();
        }
    }

    @Override
    public void rawClose() {
        System.out.println("Finish");
    }
}
