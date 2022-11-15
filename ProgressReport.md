# Progress Report

### Team

* Kevin Vece (vece2@illinois.edu, NETID: vece2)

### System

* Youtube

### Technologies

* Tauri
* React

### Updates

##### Change in Direction

The original project proposal submission was based around the idea of updating the MeTA toolkit and documenting the updates with corresponding python code. Shortly after the proposal, we began initial analysis of the parts of the MeTA C library that was not being built properly by more modern versions of runtime to discover the root cause. However, the goodwill and efforts of another student already provided a solution to update the library and shared it via campuswire with a submitted PR/fork on github. As this was the primary motivation for this project proposal, we have decided to explore other options.

##### New Project

As such, we have begun looking the the work necessary to develop an application with a focus on language learning. First, we will develop a webcrawler to scrape youtube videos and transcriptions and build a proof-of-concept (POC) database of ~1000 videos and transcripts with matching native and target languages. Then we will have implement BM25 or another algorithm on a sentence or paragraph level to find relevant segments of text to match native phrases to real world translations in the target language. Finally, we will use the youtube embedded video API to display the relevant segments of the video with live transcripts.

### Work So Far

At the moment, our work has mainly been exploratory in nature to come up with a direction to head in. As we are most familiar with using python to solve these types of problems, we are looking at potentially pivoting to gain strength in another language (rust). Thus, we have identified and experimented with likely tooling to accomplish the task, SeaORM for a powerful database interface and Tauri with React to display and embed the videos alongside the queries.

### Pending Tasks

We have divided the task structure into four main components that need to be developed in this order:

1. the database to store the URLs, transcripts, and time data
1. the web crawler to populate the database
1. the search algorithm to query over the data
1. the frontend to display the list of search hits and video segments

Using rust for tasks 1-3, we estimate 5 hours each for completion. The frontend will likely be a minimal react application which will likely also take another 5 hours for completion, for a total of less than 20 hours of work. Tasks 3 and 4 will overlap as they will both be a part of the tauri framework.

### Challenges

Aside from the complete pivot in project direction, we have spent a lot of effort in learning the target programming language (rust), as it has many features that are different than the languages we are most familiar with (python, C, javascript). As such, if there are any issues, then we will probably pivot to an entirely javascript solution using electron, or a backend solution using python. 
