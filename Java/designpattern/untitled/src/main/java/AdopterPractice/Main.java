package AdopterPractice;

import java.io.IOException;

public class Main {
    public static void main(String[] args) {
        FileIO f = new FileProperties();
        try {
            f.readFromFile("file.txt");
            f.setValue("width","1024");
            f.setValue("height","512");
            f.setValue("depth","32");
            f.writeToFile("new-file.txt");
        }catch (IOException e){
            e.printStackTrace();
        }
    }
}
