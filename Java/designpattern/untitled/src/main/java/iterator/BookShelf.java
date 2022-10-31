package iterator;

import java.util.Iterator;
import java.util.ArrayList;

public class BookShelf implements Iterable<Book>{
    private final ArrayList<Book> books;
    public BookShelf(int initialize){
        this.books = new ArrayList<Book>(initialize);
    }
    public Book getBookAt(int index){
        return books.get(index);
    }
    public void appendBook(Book book){
        books.add(book);
    }
    public int getLength(){
        return books.size();
    }
    @Override
    public Iterator<Book> iterator(){
        return new BookShelfIterator(this);
    }
}
