package projava;

public class RevRunLengthCompression {
    public static void main(String[] args) {
        var data = "ab0c1ba2bc9c0d1";
        var ans = "";
        char prev = 'a';
        for (var ch:data.toCharArray()){
            if (Character.isDigit(ch)){
                ans += String.valueOf(prev).repeat(Character. getNumericValue(ch)+1);
            }else{
                ans += String.valueOf(ch);
            }
            prev = ch;
        }
        System.out.println(ans);
    }
}
