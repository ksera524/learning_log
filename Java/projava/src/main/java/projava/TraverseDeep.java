package projava;

public class TraverseDeep {
    public static void main(String[] args) {
        int[][] map = {
                {1,1,1,1,1,1,1},
                {1,0,1,0,0,0,1},
                {1,0,0,0,1,1,1},
                {1,0,1,0,0,2,1},
                {1,1,1,1,1,1,1}
        };
        travase(map,1,1);
        char[] ch = {'.','*','G','o'};
        for (int[] row:map){
            for (int cell:row){
                System.out.print(ch[cell]);
            }
            System.out.println();
        }
    }
    static boolean travase(int[][] map,int curX,int curY){
        switch (map[curY][curX]){
            case 0:break;
            case 2:return true;
            default:return false;
        }
        map[curY][curX] = 3;
        if (travase(map,curX+1,curY) ||
                travase(map,curX-1,curY) ||
                travase(map,curX,curY+1) ||
                travase(map,curX,curY-1)){
            return true;
        }

        map[curY][curX] = 0;
        return false;
    }
}
