**App boundaries are wrong**
One bit of content per view is correct. The WM — not the app — is the right layer to solve spatial arrangement and relational clusterization. Apps smuggle in their own spatial logic (panes, splits, tabs) because WMs have historically been too dumb to handle it. A WM that's smart enough makes app-internal layout mostly redundant.

Open question: toolbars. There's often strong coupling between specialist verbs and the content they operate on — unclear whether those belong as their own views or stay bound to the content view.
