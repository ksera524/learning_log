package Composite;

public abstract class Entry {
    public abstract String getName();
    public abstract int getSIze();

    public void printList(){
        printList("");
    }
    protected abstract void printList(String prefix);

    @Override
    public String toString() {
        return getName() + "(" + getSIze() + ")";
    }
}
