# TCG-Card-Game-Server
It's for TCG Card Game Server

# Director's Note
Developers ultimately need to learn how to collaborate. I wanted to conduct a project of an appropriate size where we could all learn together why we need to consider the domain perspective when developing software, the importance of communication, the importance of documentation, and the importance of issuing issues. We decided that this would be the most appropriate way to secure meaningful project results while carrying out all of the above in a short period of time.  

A chronic problem in the embedded and hardware fields is collaboration. SW developers who cannot collaborate, no matter how much experience they have, reduce productivity in creating something together. The purpose is to gain experience in collaborative and large-scale software development.  

And finally we did it. We have confirmed that not only web/app development but also game development can achieve high productivity at low cost using agile methods.

# Demo Play
[![Video Label](http://img.youtube.com/vi/EQCNLK_O2JE/0.jpg)](https://youtu.be/EQCNLK_O2JE)

# Review
In the case of the server, I think the domains were separated quite well and the flexibility to change was quite excellent, so whenever a change occurred in the UI, I was able to respond to the UI situation very quickly.
By performing DDD, we wanted to achieve some degree of flexibility while complying with the SRP rules, and I am glad that we have achieved this to some extent.
The disappointing part is that the UI support was busy so it could not be processed completely asynchronously. So, the request is processed as Blocking from your perspective. On the other hand, the Opponent perspective is processed as Non-Blocking. Fortunately, the response speed is not slow, so you can play with some level of comfort, but blocking problems exist in situations where traffic increases. Next time I develop, I want to make this part non-blocking as well to perform completely asynchronous processing.  

# Project Manager
[![Sanghoon Lee]()](https://github.com/silenc3502)  

# Server Team
[![Sanghoon Lee]()](https://github.com/silenc3502)  
[![Janghun Park]()]()  
[![Sungyong Lee]()]()  
[![Youngchan Hwang]()]()  
[![Hyoungjun Lee]()]()  
[![Sanggun Youn]()]()  

# Client Team
[![Sanghoon Lee]()](https://github.com/silenc3502)  
[![Jaeseung Lee]()]()  
[![Seunghun Jo]()]()  
[![Jaelim Lee]()]()  
[![Junghun Woo]()]() 

# Design
[![Jaelim Lee]()]()  

# Animation
[![Sanggun Youn]()]()  
[![Jaeseung Lee]()]()  
[![Sanghoon Lee]()](https://github.com/silenc3502)  

# Sound Engineering
[![Janghun Park]()]()  
