package SingletonPractice;

public class TicketMaker {
    private static final TicketMaker ticketMaker = new TicketMaker();
    private int ticket = 100;

    public static TicketMaker getInstance(){
        return ticketMaker;
    }

    public int getNextTicketNumber(){
        return ticket++;
    }
}
