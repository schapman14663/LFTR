Quick and Dirty for now:

To date, I've been using a spreadsheet to try and track my weightlifting progress.

There's some problems with this though:

- It's a lot of maintenance. Sure, I could probably, sit down and set up some conditional formatting and formulas and stuff,
but first, I want to *try* to create something of my own.

- I've found that the Sheet gets actually quite unwieldy (for me) because of the stuff I want to track. At the moment I don't make notes on it about how a set **felt**,
which certainly seems like something I should be doing.

- My progression system (we listen and we don't judge) is based on hitting (or not) a certain rep/rep range, and it doesn't automatically fill in *next weeks* weight
when I note down my reps, so I have to go through the whole thing to update it over the weekend, which, I.. I may or may not work out, like, 5 - 6 days a week. (we listen. and we don't judge.) for *some* time. (we. **listen**. and we don't. **judge**).

So yeah I'm building a *something* that I'm hoping I can use to replace my dumb spreadsheet.

Structure Sketching:
> A Block (of x Weeks)
>> Made up of y sessions per week
>>> Made up of z exercises per session

y'know, normal stuff.

A Block:

- Has a Set Start and End date
- \[Optional\] Deload Weeks
- As above, each week is a list of sessions.

A Session:

- As above, basically a list of exercises.

As Exercise:

- Name
- Set Up Notes (Seat Height, Bench Angle etc...)
- Additional Equipment (Straps, Wraps, Sleeves, Belts, etc...)
- Has Sets
- Sets have:
either:
- a time limit
or
- - Weights
- - Reps
- - - Target Reps
- - - - Low Target
- - - - High Target
- - - Actual Reps
- \[Optional] Progression Instructions
- Notes
- Rest Timer
