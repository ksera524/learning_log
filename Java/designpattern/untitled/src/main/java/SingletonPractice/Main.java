package SingletonPractice;

public class Main {
    public static void main(String[] args) {
        TicketMaker ticketMaker = TicketMaker.getInstance();
        System.out.println(ticketMaker.getNextTicketNumber());
        TicketMaker ticketMaker1 = TicketMaker.getInstance();
        System.out.println(ticketMaker1.getNextTicketNumber());
    }
}
