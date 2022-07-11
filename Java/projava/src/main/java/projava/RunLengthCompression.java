package projava;

public class RunLengthCompression {
    public static void main(String[] args) {
        final var COUNTER_BAS=-1;
        var data = "abbcccbaaaabccccccccccccddd";

        var count = COUNTER_BAS;
        String prev = "";
        var ans = "";
        for(String ch:data.split("")){
            if (prev.equals(ch)){
                count++;
                if (count==9){
                    ans += prev;
                    ans += count;
                    count = COUNTER_BAS;
                }
            }else{
                ans += prev;
                if(count >= 0){
                    ans += count;
                    count = COUNTER_BAS;
                }
            }
            prev = ch;
        }
        if(count >= 0){
            ans += prev;
            ans += count;
        }
        System.out.println(ans);
    }
}
