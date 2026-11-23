public class Driver {
    public static void main(String[] args) throws InterruptedException{
        Runnable task = () -> {
            for (int i = 1; i <= 5; i++) {
                System.out.println("Worker thread: " + i);
                try {
                    Thread.sleep(500); 
                } catch (InterruptedException e) {}
            }
        };

        Thread t1 = new Thread(task);

        t1.start();

        System.out.println("Main launched the thread");
        
        t1.join();
        System.out.println("Finished main");
    }
}