# SyncedLabrinth

This begins the readMe;

For this Assignment we have 2 programs.

Program 1: The Labyrinth
	function: birthday/cupcake()

	The theory behind this problem is similar to that of the guardian prisoner problem discussed in class
	For use to keep count of everyone having passed through the labyrinth, we will want a single person
	to act as a counter. They will act opposite the rest of the other persons: In the scenario that there 
	is no cupcake, they will add the cupcake again and count another participant as added. 
		For the rest of the threads, we will only touch the cupcake once. And that is on our first 
	visit where we see that there is a cupcake. Henceforth we will not interfere with other threads, as
	each person only get's 1 cupcake. Once the main branch hits the counter that thread will finish.
		Since the main thread is only waiting on the counter thread, we will clse all other threads once we
	finish main, thus why we use .join() only to wait on the counter;

Program 2: The Museum
	function: shworoom/attempt()
	For this one, the implementation is quite the same. But it adds another layer of complexity. A queue.
	The difference here is that, the queue will tell the thread which thread to retrieve next.

Note: park() is used to sleep a thread until called upon. In contrast to using a busywait


For problem 2, the use of a queue is the most efficient option because it can maintain itself in a timely manner.
	In contrast, for a regular lock, that would create an audience waiting right outside of the door. If that were
	the case, it wouldn't be certain who would get to be next, as all the 'guests' are all waiting. And using a backoff
	lock as an option while more helpful is actually also not too efficient. It follows the worry of starvation. Every
	time that the 'guest' may attempt to visit, the room may always be full at that time, leading the thread to not be able
	to visit. With the queue, every thread that desires to view the vase will be able to see it; and in a sequential order.