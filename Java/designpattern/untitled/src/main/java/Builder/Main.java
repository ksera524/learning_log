package Builder;

import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        Scanner scan = new Scanner(System.in);
        String text = scan.nextLine();
        scan.close();

        if (text.equals("text")){
            TextBuilder builder = new TextBuilder();
            Director director = new Director(builder);
            director.construct();
            String result = builder.getTextResult();
            System.out.println(result);
        } else if(text.equals("html")){
            HTMLBuilder builder = new HTMLBuilder();
            Director director = new Director(builder);
            director.construct();
            String result = builder.getHTMLResult();
            System.out.println(result);
        } else {
            System.out.println("入力がおかしい");
        }
    }
}
