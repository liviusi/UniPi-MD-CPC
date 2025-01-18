# Hands-on 3

## Exercise 1
As the last hands-on of your class is due to two weeks after Christmas, you are ready to plan your Christmas holiday traveling around Europe, to visit different cities. You have a tour guide to Europe, which presents a different itinerary for each city. Each itinerary specifies how many different attractions can be visited per day. As an example, this is the itinerary for Florence

| Day  | City |
 ----- | -----
| 1    | 3    |
| 2    | 2    |
| 3    | 1    |
| 4    | 4    |

This means that if you spend two days in Florence you will have the chance to visit
3 + 2 = 5 different attractions.
You want to visit as many attractions as you can, considering that you only have a limited number of days on vacation before the oral exam. Your task is to write a program to organize your holiday. Note that you can visit the attractions in the order provided by the guide, meaning that if you spend one day in Florence you will visit attractions (i.e., you cannot "cherry pick" the 4 attractions of the last day).

You are provided with the number of attractions you can visit for each of the
D days, in each city. The number of cities is n. Your goal is to identify the maximum number of attractions the tourist can visit. The time complexity of your solution should be O(DnD).

## Exercise 2
As this problem seems very challenging, you might be tempted to cancel your Christmas holiday plans to travel around Europe done in the previous problem. If luck is on your side, you’ll still have a couple of days to visit Massaciuccoli lake.

A poor professor in the city of straightening tower is tasked with preparing a new course. Armed with a list of potential topics to choose from, he knows the beauty
b[i] and the difficulty d[i] of topic i.

As students can be picky (I’m joking, if they happen to be permalous too!), they appreciate a course only if each lecture is more beautiful than the previous one. Moreover, adhering to pedagogical principles, the topics must exhibit increasing levels of difficulty.

The poor professor’s objective is to select the maximum number of topics for his upcoming course.

Your challenge is to devise an efficient algorithm to determine this maximum number of selected topics.