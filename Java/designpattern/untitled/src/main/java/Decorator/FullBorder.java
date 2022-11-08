package Decorator;

public class FullBorder extends Border{
    public FullBorder(Display display){
        super(display);
    }

    @Override
    public int getColumns() {
        return display.getColumns() + 2;
    }

    @Override
    public int getRows() {
        return display.getRows()+2;
    }

    @Override
    public String getRowText(int row) {
        if (row == 0){
            return "+".repeat(display.getColumns() + 2);
        } else if (row == display.getRows() + 1){
            return "+".repeat(display.getColumns() + 2);
        } else {
            return "|" + display.getRowText(row - 1) + "|";
        }
    }
}
