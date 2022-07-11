package projava;

import java.util.ArrayList;

import static java.lang.Math.*;

public class RemoveDuplicate {
    public static void main(String[] args) {
        var data = "abcccbaabcc".split("");
        var ans = "";
        for (int i = 0; i < data.length; i++) {
            if (i > 0 && data[i].equals(data[i - 1])) {
                continue;
            }
            ans += data[i];
        }
        System.out.println(ans);

        ans = "";
        for (int i = 0; i < data.length; i += 2) {
            if (i + 1 >= data.length) {
                ans += data[i];
                continue;
            }
            ans += data[i + 1] + data[i];
        }
        System.out.println(ans);

        int arr[] = {3, 6, 9, 4, 2, 1, 5};
        var ansArr = new ArrayList<Integer>();
        for (int i = 0; i < arr.length; i++) {
            if(i == arr.length-1){
                ansArr.add(arr[i]);
                continue;
            }
            ansArr.add(max(arr[i],arr[i+1]));
        }
        System.out.println(ansArr);
    }
}
